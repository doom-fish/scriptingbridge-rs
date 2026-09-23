use crate::{Result, ScriptingBridgeError};

const MAX_NAME_LEN: usize = 255;

const DENIED_NAMES: &[&str] = &[
    "addObserver",
    "allowsWeakReference",
    "autorelease",
    "awakeAfterUsingCoder",
    "class",
    "dealloc",
    "didChangeValueForKey",
    "doesNotRecognizeSelector",
    "finalize",
    "forwardInvocation",
    "forwardingTargetForSelector",
    "instanceMethodForSelector",
    "methodForSelector",
    "methodSignatureForSelector",
    "mutableArrayValueForKey",
    "mutableArrayValueForKeyPath",
    "mutableOrderedSetValueForKey",
    "mutableOrderedSetValueForKeyPath",
    "mutableSetValueForKey",
    "mutableSetValueForKeyPath",
    "performSelector",
    "performSelectorInBackground",
    "performSelectorOnMainThread",
    "release",
    "removeObserver",
    "retain",
    "retainCount",
    "retainWeakReference",
    "sendEvent",
    "superclass",
    "valueForUndefinedKey",
    "willChangeValueForKey",
    "zone",
];

const DENIED_FAMILIES: &[&str] = &["alloc", "copy", "init", "mutableCopy", "new"];

fn rejected(function: &'static str, name: &str, reason: &str) -> ScriptingBridgeError {
    ScriptingBridgeError::new(
        function,
        format!("selector `{name}` is not allowed: {reason}"),
    )
}

fn in_family(name: &str, family: &str) -> bool {
    name.strip_prefix(family)
        .is_some_and(|rest| !rest.starts_with(|c: char| c.is_ascii_lowercase()))
}

fn check_name(name: &str, function: &'static str) -> Result<()> {
    if name.is_empty() {
        return Err(rejected(function, name, "it is empty"));
    }
    if name.len() > MAX_NAME_LEN {
        return Err(rejected(function, name, "it is too long"));
    }
    if !name.starts_with(|c: char| c.is_ascii_alphabetic()) {
        return Err(rejected(
            function,
            name,
            "it must start with an ASCII letter",
        ));
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(rejected(
            function,
            name,
            "only ASCII letters, digits and '_' are accepted",
        ));
    }
    if DENIED_NAMES.contains(&name) {
        return Err(rejected(
            function,
            name,
            "it manages memory or dispatches selectors",
        ));
    }
    if let Some(family) = DENIED_FAMILIES
        .iter()
        .find(|family| in_family(name, family))
    {
        return Err(rejected(
            function,
            name,
            &format!("it belongs to the `{family}` method family"),
        ));
    }
    if name
        .strip_prefix("set")
        .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_uppercase()))
    {
        return Err(rejected(
            function,
            name,
            "setters are only reachable through dedicated methods such as `set_to`",
        ));
    }
    Ok(())
}

pub(crate) fn check_command(command: &str, function: &'static str) -> Result<()> {
    check_name(command, function)
}

pub(crate) fn check_selector(selector: &str, arity: usize, function: &'static str) -> Result<()> {
    let name = match arity {
        0 => selector,
        1 => selector.strip_suffix(':').ok_or_else(|| {
            rejected(
                function,
                selector,
                "a one-argument selector must end with ':'",
            )
        })?,
        _ => {
            return Err(rejected(
                function,
                selector,
                "only zero- or one-argument selectors are supported",
            ))
        }
    };
    if name.contains(':') {
        return Err(rejected(
            function,
            selector,
            "it takes a different number of arguments",
        ));
    }
    check_name(name, function)
}

pub(crate) fn check_key_path(key_path: &str, function: &'static str) -> Result<()> {
    key_path
        .split('.')
        .try_for_each(|component| check_name(component, function))
}

#[cfg(test)]
mod tests {
    use super::{check_command, check_key_path, check_selector};

    const FUNCTION: &str = "test";

    #[test]
    fn memory_management_selectors_are_rejected() {
        for name in [
            "dealloc",
            "release",
            "retain",
            "autorelease",
            "retainCount",
            "finalize",
            "zone",
        ] {
            assert!(check_command(name, FUNCTION).is_err(), "{name}");
            assert!(check_selector(name, 0, FUNCTION).is_err(), "{name}");
            assert!(check_key_path(name, FUNCTION).is_err(), "{name}");
        }
    }

    #[test]
    fn retained_result_families_are_rejected() {
        for name in [
            "alloc",
            "allocWithZone",
            "new",
            "newDocument",
            "copy",
            "copyWithZone",
            "copyItems",
            "mutableCopy",
            "mutableCopyWithZone",
            "init",
            "initWithName",
        ] {
            assert!(check_command(name, FUNCTION).is_err(), "{name}");
        }
        for name in ["copyright", "newest", "news", "initialize", "allocation"] {
            assert!(check_command(name, FUNCTION).is_ok(), "{name}");
        }
    }

    #[test]
    fn setters_and_dispatchers_are_rejected() {
        for selector in [
            "setValue:",
            "setTo:",
            "setTimeout:",
            "performSelector:",
            "awakeAfterUsingCoder:",
            "methodSignatureForSelector:",
            "forwardingTargetForSelector:",
            "valueForUndefinedKey:",
        ] {
            assert!(check_selector(selector, 1, FUNCTION).is_err(), "{selector}");
        }
        assert!(check_command("settings", FUNCTION).is_ok());
        assert!(check_command("setup", FUNCTION).is_ok());
    }

    #[test]
    fn ordinary_scripting_names_are_accepted() {
        for name in [
            "name",
            "desktop",
            "disks",
            "startupDisk",
            "frontmost",
            "version",
        ] {
            assert!(check_command(name, FUNCTION).is_ok(), "{name}");
            assert!(check_selector(name, 0, FUNCTION).is_ok(), "{name}");
        }
        assert!(check_selector("valueForKey:", 1, FUNCTION).is_ok());
        assert!(check_selector("objectWithName:", 1, FUNCTION).is_ok());
        assert!(check_key_path("startupDisk.capacity", FUNCTION).is_ok());
    }

    #[test]
    fn malformed_names_are_rejected() {
        for name in [
            "",
            "_private",
            "_tryRetain",
            "1abc",
            "name:",
            "na me",
            "näme",
            "@count",
            "a.b",
        ] {
            assert!(check_command(name, FUNCTION).is_err(), "{name:?}");
        }
        assert!(check_command(&"a".repeat(256), FUNCTION).is_err());
        assert!(check_command(&"a".repeat(255), FUNCTION).is_ok());
    }

    #[test]
    fn selector_arity_must_match_the_colon_count() {
        assert!(check_selector("name", 1, FUNCTION).is_err());
        assert!(check_selector("valueForKey:", 0, FUNCTION).is_err());
        assert!(check_selector("sort:by:", 1, FUNCTION).is_err());
        assert!(check_selector("sort:by:", 2, FUNCTION).is_err());
        assert!(check_selector("::", 1, FUNCTION).is_err());
    }

    #[test]
    fn every_key_path_component_is_checked() {
        assert!(check_key_path("desktop.retain", FUNCTION).is_err());
        assert!(check_key_path("desktop..name", FUNCTION).is_err());
        assert!(check_key_path(".desktop", FUNCTION).is_err());
        assert!(check_key_path("desktop.", FUNCTION).is_err());
        assert!(check_key_path("desktop._name", FUNCTION).is_err());
        assert!(check_key_path("desktop.@count", FUNCTION).is_err());
    }
}
