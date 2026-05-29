// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "ScriptingBridgeBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "ScriptingBridgeBridge",
            type: .static,
            targets: ["ScriptingBridgeBridge"])
    ],
    targets: [
        .target(
            name: "ScriptingBridgeBridge",
            path: "Sources/ScriptingBridgeBridge")
    ]
)
