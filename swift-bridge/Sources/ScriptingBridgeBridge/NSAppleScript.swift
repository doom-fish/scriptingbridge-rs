import Foundation

final class SBRSAppleScriptHandle: NSObject {
  let script: NSAppleScript

  init(script: NSAppleScript) {
    self.script = script
  }
}

private func sbAppleScriptErrorMessage(_ errorInfo: NSDictionary?) -> String {
  guard let errorInfo else {
    return "AppleScript failed without error details"
  }

  let entries = errorInfo
    .compactMap { key, value -> String? in
      guard let key = key as? String else {
        return nil
      }
      return "\(key)=\(value)"
    }
    .sorted()

  if entries.isEmpty {
    return "AppleScript failed without error details"
  }

  return entries.joined(separator: "; ")
}

@_cdecl("sb_apple_script_create_with_source")
public func sb_apple_script_create_with_source(
  _ sourcePointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let sourcePointer else {
    sbSetError(errorOut, "missing AppleScript source")
    return nil
  }

  guard let script = NSAppleScript(source: String(cString: sourcePointer)) else {
    sbSetError(errorOut, "NSAppleScript could not parse the provided source")
    return nil
  }

  return sbRetain(SBRSAppleScriptHandle(script: script))
}

@_cdecl("sb_apple_script_create_with_contents_of_url")
public func sb_apple_script_create_with_contents_of_url(
  _ urlPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let urlPointer else {
    sbSetError(errorOut, "missing AppleScript URL")
    return nil
  }

  let rawURL = String(cString: urlPointer)
  guard let url = sbURL(from: rawURL) else {
    sbSetError(errorOut, "could not parse AppleScript URL \(rawURL)")
    return nil
  }

  var errorInfo: NSDictionary?
  guard let script = NSAppleScript(contentsOf: url, error: &errorInfo) else {
    sbSetError(errorOut, sbAppleScriptErrorMessage(errorInfo))
    return nil
  }

  return sbRetain(SBRSAppleScriptHandle(script: script))
}

@_cdecl("sb_apple_script_source")
public func sb_apple_script_source(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSAppleScriptHandle = sbBorrow(rawHandle)
  return handle.script.source.flatMap(sbCString)
}

@_cdecl("sb_apple_script_is_compiled")
public func sb_apple_script_is_compiled(_ rawHandle: UnsafeMutableRawPointer?) -> Bool {
  guard let rawHandle else {
    return false
  }

  let handle: SBRSAppleScriptHandle = sbBorrow(rawHandle)
  return handle.script.isCompiled
}

@_cdecl("sb_apple_script_compile")
public func sb_apple_script_compile(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleScript handle")
    return false
  }

  let handle: SBRSAppleScriptHandle = sbBorrow(rawHandle)
  var errorInfo: NSDictionary?
  let didCompile = handle.script.compileAndReturnError(&errorInfo)
  if !didCompile {
    sbSetError(errorOut, sbAppleScriptErrorMessage(errorInfo))
  }
  return didCompile
}

@_cdecl("sb_apple_script_execute")
public func sb_apple_script_execute(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleScript handle")
    return nil
  }

  let handle: SBRSAppleScriptHandle = sbBorrow(rawHandle)
  var errorInfo: NSDictionary?
  let descriptor = handle.script.executeAndReturnError(&errorInfo)
  if errorInfo != nil {
    sbSetError(errorOut, sbAppleScriptErrorMessage(errorInfo))
    return nil
  }

  return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: descriptor))
}

@_cdecl("sb_apple_script_execute_apple_event")
public func sb_apple_script_execute_apple_event(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ eventHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleScript handle")
    return nil
  }
  guard let event = sbDescriptor(fromHandle: eventHandle) else {
    sbSetError(errorOut, "missing Apple event descriptor")
    return nil
  }

  let handle: SBRSAppleScriptHandle = sbBorrow(rawHandle)
  var errorInfo: NSDictionary?
  let descriptor = handle.script.executeAppleEvent(event, error: &errorInfo)
  if errorInfo != nil {
    sbSetError(errorOut, sbAppleScriptErrorMessage(errorInfo))
    return nil
  }

  return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: descriptor))
}

@_cdecl("sb_apple_script_release")
public func sb_apple_script_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
