import AppKit
import Foundation
import ScriptingBridge

final class SBRSApplicationHandle: NSObject {
  let bundleIdentifier: String
  let application: SBApplication

  init(bundleIdentifier: String, application: SBApplication) {
    self.bundleIdentifier = bundleIdentifier
    self.application = application
  }
}

final class SBRSObjectHandle: NSObject {
  let object: SBObject

  init(object: SBObject) {
    self.object = object
  }
}

final class SBRSElementArrayHandle: NSObject {
  let array: SBElementArray

  init(array: SBElementArray) {
    self.array = array
  }
}

private func sbSetError(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  _ message: String
) {
  errorOut?.pointee = sbCString(message)
}

private func sbPerform(
  application: SBApplication,
  command: String,
  argument: String?,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Any? {
  let selectorName = argument == nil ? command : "\(command):"
  let selector = NSSelectorFromString(selectorName)

  if application.responds(to: selector) {
    if let argument {
      return application.perform(selector, with: argument as NSString)?.takeUnretainedValue()
    }
    return application.perform(selector)?.takeUnretainedValue()
  }

  if argument == nil {
    return application.value(forKeyPath: command)
  }

  sbSetError(errorOut, "application does not respond to selector \(selectorName)")
  return nil
}

@_cdecl("sb_application_create")
public func sb_application_create(
  _ bundleIdentifierPtr: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let bundleIdentifierPtr else {
    sbSetError(errorOut, "missing bundle identifier")
    return nil
  }
  let bundleIdentifier = String(cString: bundleIdentifierPtr)
  guard let application = SBApplication(bundleIdentifier: bundleIdentifier) else {
    sbSetError(errorOut, "could not create SBApplication for \(bundleIdentifier)")
    return nil
  }
  return sbRetain(SBRSApplicationHandle(bundleIdentifier: bundleIdentifier, application: application))
}

@_cdecl("sb_application_is_running")
public func sb_application_is_running(_ rawHandle: UnsafeMutableRawPointer?) -> Bool {
  guard let rawHandle else { return false }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return handle.application.isRunning
}

@_cdecl("sb_application_launch")
public func sb_application_launch(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let url = NSWorkspace.shared.urlForApplication(withBundleIdentifier: handle.bundleIdentifier) else {
    sbSetError(errorOut, "could not resolve application URL for \(handle.bundleIdentifier)")
    return false
  }

  let sem = DispatchSemaphore(value: 0)
  var didLaunch = false
  Task {
    do {
      _ = try await NSWorkspace.shared.openApplication(
        at: url,
        configuration: NSWorkspace.OpenConfiguration())
      didLaunch = true
    } catch {
      sbSetError(errorOut, (error as NSError).localizedDescription)
    }
    sem.signal()
  }

  let waitResult = sem.wait(timeout: .now() + .seconds(30))
  if waitResult == .timedOut {
    sbSetError(errorOut, "timed out while launching \(handle.bundleIdentifier)")
    return false
  }
  return didLaunch
}

@_cdecl("sb_application_activate")
public func sb_application_activate(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  handle.application.activate()
  return true
}

@_cdecl("sb_application_quit")
public func sb_application_quit(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  _ = sbPerform(application: handle.application, command: "quit", argument: nil, errorOut: errorOut)
  return errorOut?.pointee == nil
}

@_cdecl("sb_application_terminate")
public func sb_application_terminate(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  let running = NSRunningApplication.runningApplications(withBundleIdentifier: handle.bundleIdentifier)
  guard let app = running.first else {
    return true
  }
  if app.terminate() {
    return true
  }
  sbSetError(errorOut, "terminate() returned false for \(handle.bundleIdentifier)")
  return false
}

@_cdecl("sb_application_tell")
public func sb_application_tell(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ command: UnsafePointer<CChar>?,
  _ argument: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let command else {
    sbSetError(errorOut, "missing tell command")
    return nil
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  let result = sbPerform(
    application: handle.application,
    command: String(cString: command),
    argument: argument.map { String(cString: $0) },
    errorOut: errorOut)
  guard let result else { return nil }
  return sbCString(String(describing: result))
}

@_cdecl("sb_application_object_for_key_path")
public func sb_application_object_for_key_path(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyPath: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let keyPath else {
    sbSetError(errorOut, "missing key path")
    return nil
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let value = handle.application.value(forKeyPath: String(cString: keyPath)) else {
    return nil
  }
  guard let object = value as? SBObject else {
    sbSetError(errorOut, "key path did not resolve to an SBObject")
    return nil
  }
  return sbRetain(SBRSObjectHandle(object: object))
}

@_cdecl("sb_application_element_array_for_key_path")
public func sb_application_element_array_for_key_path(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyPath: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let keyPath else {
    sbSetError(errorOut, "missing key path")
    return nil
  }
  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let value = handle.application.value(forKeyPath: String(cString: keyPath)) else {
    return nil
  }
  guard let array = value as? SBElementArray else {
    sbSetError(errorOut, "key path did not resolve to an SBElementArray")
    return nil
  }
  return sbRetain(SBRSElementArrayHandle(array: array))
}

@_cdecl("sb_application_release")
public func sb_application_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else { return }
  sbRelease(rawHandle)
}

@_cdecl("sb_object_description")
public func sb_object_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else { return nil }
  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  return sbCString(handle.object.description)
}

@_cdecl("sb_object_get_description")
public func sb_object_get_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else { return nil }
  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  guard let value = handle.object.get() else { return nil }
  return sbCString(String(describing: value))
}

@_cdecl("sb_object_last_error_description")
public func sb_object_last_error_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else { return nil }
  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  guard let error = handle.object.lastError() else { return nil }
  return sbCString((error as NSError).localizedDescription)
}

@_cdecl("sb_object_release")
public func sb_object_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else { return }
  sbRelease(rawHandle)
}

@_cdecl("sb_element_array_description")
public func sb_element_array_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else { return nil }
  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  return sbCString(handle.array.description)
}

@_cdecl("sb_element_array_get_description")
public func sb_element_array_get_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else { return nil }
  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  guard let value = handle.array.get() else { return nil }
  return sbCString(String(describing: value))
}

@_cdecl("sb_element_array_release")
public func sb_element_array_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else { return }
  sbRelease(rawHandle)
}
