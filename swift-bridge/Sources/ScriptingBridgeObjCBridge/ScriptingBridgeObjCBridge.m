#import "ScriptingBridgeObjCBridge.h"

#import <ScriptingBridge/ScriptingBridge.h>
#import <objc/message.h>
#import <string.h>

static NSString *SBRSExceptionMessage(id exception) {
    if ([exception isKindOfClass:[NSException class]]) {
        NSException *raised = exception;
        return [NSString stringWithFormat:@"%@: %@", raised.name, raised.reason ?: @"no reason given"];
    }
    return [NSString stringWithFormat:@"Objective-C exception: %@", exception];
}

static const char *SBRSSkipTypeQualifiers(const char *type) {
    while (*type != '\0' && strchr("rnNoORV", *type) != NULL) {
        type++;
    }
    return type;
}

static BOOL SBRSIsObjectType(const char *type) {
    type = SBRSSkipTypeQualifiers(type);
    return type[0] == '@' && type[1] != '?';
}

static BOOL SBRSIsVoidType(const char *type) {
    type = SBRSSkipTypeQualifiers(type);
    return type[0] == 'v' && type[1] == '\0';
}

static NSUInteger SBRSSelectorArity(NSString *selectorName) {
    return [selectorName componentsSeparatedByString:@":"].count - 1;
}

static BOOL SBRSCheckMethod(
    id target,
    SEL selector,
    NSUInteger arity,
    SBRSReturnKind returnKind,
    BOOL *returnsObject,
    NSString **message) {
    NSString *name = NSStringFromSelector(selector);
    if (![target respondsToSelector:selector]) {
        *message = [NSString stringWithFormat:@"%@ does not respond to %@", [target class], name];
        return NO;
    }
    NSMethodSignature *signature = [target methodSignatureForSelector:selector];
    if (signature == nil) {
        *message = [NSString stringWithFormat:@"%@ has no method signature for %@", [target class], name];
        return NO;
    }
    if (signature.numberOfArguments != arity + 2) {
        *message = [NSString stringWithFormat:@"%@ takes %lu arguments, not %lu", name,
                                              (unsigned long)(signature.numberOfArguments - 2), (unsigned long)arity];
        return NO;
    }
    for (NSUInteger index = 2; index < signature.numberOfArguments; index++) {
        if (!SBRSIsObjectType([signature getArgumentTypeAtIndex:index])) {
            *message = [NSString stringWithFormat:@"argument %lu of %@ is not an object", (unsigned long)(index - 1), name];
            return NO;
        }
    }
    const char *returnType = signature.methodReturnType;
    BOOL isObject = SBRSIsObjectType(returnType);
    BOOL isVoid = SBRSIsVoidType(returnType);
    BOOL accepted = (returnKind == SBRSReturnKindObject && isObject) ||
                    (returnKind == SBRSReturnKindVoid && isVoid) ||
                    (returnKind == SBRSReturnKindObjectOrVoid && (isObject || isVoid));
    if (!accepted) {
        *message = [NSString stringWithFormat:@"%@ returns '%s', which this call does not accept", name, returnType];
        return NO;
    }
    *returnsObject = isObject;
    return YES;
}

static BOOL SBRSCheckGetter(id target, SEL selector, NSString **message) {
    NSMethodSignature *signature = [target methodSignatureForSelector:selector];
    if (signature == nil || signature.numberOfArguments != 2 || SBRSIsVoidType(signature.methodReturnType)) {
        *message = [NSString stringWithFormat:@"%@ is not a readable key of %@", NSStringFromSelector(selector), [target class]];
        return NO;
    }
    return YES;
}

static BOOL SBRSCheckKey(id target, NSString *key, NSString **message) {
    if ([target isKindOfClass:[NSDictionary class]]) {
        return YES;
    }
    if ([target isKindOfClass:[SBElementArray class]]) {
        return SBRSCheckKey([(SBElementArray *)target objectAtIndex:0], key, message);
    }
    if ([target isKindOfClass:[NSArray class]]) {
        for (id element in (NSArray *)target) {
            if (element != [NSNull null] && !SBRSCheckKey(element, key, message)) {
                return NO;
            }
        }
        return YES;
    }
    SEL selector = NSSelectorFromString(key);
    if (![target respondsToSelector:selector]) {
        *message = [NSString stringWithFormat:@"%@ has no key %@", [target class], key];
        return NO;
    }
    if (!SBRSCheckGetter(target, selector, message)) {
        return NO;
    }
    NSString *capitalized = [[[key substringToIndex:1] uppercaseString] stringByAppendingString:[key substringFromIndex:1]];
    SEL prefixed = NSSelectorFromString([@"get" stringByAppendingString:capitalized]);
    return ![target respondsToSelector:prefixed] || SBRSCheckGetter(target, prefixed, message);
}

static BOOL SBRSIsKeyValueSelector(SEL selector) {
    return selector == @selector(valueForKey:) || selector == @selector(valueForKeyPath:);
}

static NSString *_Nullable SBRSSingleKey(id _Nullable argument, NSString **message) {
    if (![argument isKindOfClass:[NSString class]] || [(NSString *)argument length] == 0 ||
        [(NSString *)argument containsString:@"."]) {
        *message = @"a key-value coding selector needs a single non-empty key string";
        return nil;
    }
    return argument;
}

BOOL SBRSInvoke(
    id target,
    NSString *selectorName,
    id _Nullable argument,
    BOOL hasArgument,
    SBRSReturnKind returnKind,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        if (hasArgument && SBRSIsKeyValueSelector(NSSelectorFromString(selectorName))) {
            NSString *key = SBRSSingleKey(argument, message);
            return key != nil && SBRSValueForKeyPath(target, key, result, message);
        }
        NSUInteger arity = hasArgument ? 1 : 0;
        if (SBRSSelectorArity(selectorName) != arity) {
            *message = [NSString stringWithFormat:@"%@ does not take %lu arguments", selectorName, (unsigned long)arity];
            return NO;
        }
        SEL selector = NSSelectorFromString(selectorName);
        BOOL returnsObject = NO;
        if (!SBRSCheckMethod(target, selector, arity, returnKind, &returnsObject, message)) {
            return NO;
        }
        if (returnsObject) {
            id value = hasArgument ? ((id (*)(id, SEL, id))objc_msgSend)(target, selector, argument)
                                   : ((id (*)(id, SEL))objc_msgSend)(target, selector);
            *result = value;
        } else {
            if (hasArgument) {
                ((void (*)(id, SEL, id))objc_msgSend)(target, selector, argument);
            } else {
                ((void (*)(id, SEL))objc_msgSend)(target, selector);
            }
            *result = nil;
        }
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}

BOOL SBRSValueForKeyPath(
    id target,
    NSString *keyPath,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        id current = target;
        for (NSString *key in [keyPath componentsSeparatedByString:@"."]) {
            if (current == nil || current == [NSNull null]) {
                *result = nil;
                return YES;
            }
            if (key.length == 0) {
                *message = [NSString stringWithFormat:@"key path %@ has an empty component", keyPath];
                return NO;
            }
            if (!SBRSCheckKey(current, key, message)) {
                return NO;
            }
            current = [current valueForKey:key];
        }
        *result = current;
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}

BOOL SBRSArrayByApplyingSelector(
    id array,
    NSString *selectorName,
    id _Nullable argument,
    BOOL hasArgument,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        if (![array isKindOfClass:[SBElementArray class]]) {
            *message = [NSString stringWithFormat:@"%@ is not an SBElementArray", [array class]];
            return NO;
        }
        SBElementArray *elements = array;
        NSUInteger arity = hasArgument ? 1 : 0;
        if (SBRSSelectorArity(selectorName) != arity) {
            *message = [NSString stringWithFormat:@"%@ does not take %lu arguments", selectorName, (unsigned long)arity];
            return NO;
        }
        SEL selector = NSSelectorFromString(selectorName);
        BOOL returnsObject = NO;
        id probe = [elements objectAtIndex:0];
        if (!SBRSCheckMethod(probe, selector, arity, SBRSReturnKindObject, &returnsObject, message)) {
            return NO;
        }
        if (hasArgument && SBRSIsKeyValueSelector(selector)) {
            NSString *key = SBRSSingleKey(argument, message);
            if (key == nil || !SBRSCheckKey(probe, key, message)) {
                return NO;
            }
        }
        *result = hasArgument ? [elements arrayByApplyingSelector:selector withObject:argument]
                              : [elements arrayByApplyingSelector:selector];
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}

BOOL SBRSSendEvent(
    id target,
    AEEventClass eventClass,
    AEEventID eventID,
    NSArray<NSNumber *> *codes,
    NSArray *values,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        if (![target isKindOfClass:[SBObject class]]) {
            *message = [NSString stringWithFormat:@"%@ is not an SBObject", [target class]];
            return NO;
        }
        if (codes.count != values.count || codes.count > 8) {
            *message = @"sendEvent takes up to 8 matching parameter codes and values";
            return NO;
        }
        DescType c[8] = {0};
        id v[8] = {nil};
        for (NSUInteger index = 0; index < codes.count; index++) {
            c[index] = codes[index].unsignedIntValue;
            v[index] = values[index];
            if (c[index] == 0) {
                *message = @"an Apple event parameter code of 0 would end the parameter list";
                return NO;
            }
        }
        SBObject *object = target;
        const DescType end = 0;
        id value = nil;
        switch (codes.count) {
            case 0:
                value = [object sendEvent:eventClass id:eventID parameters:end];
                break;
            case 1:
                value = [object sendEvent:eventClass id:eventID parameters:c[0], v[0], end];
                break;
            case 2:
                value = [object sendEvent:eventClass id:eventID parameters:c[0], v[0], c[1], v[1], end];
                break;
            case 3:
                value = [object sendEvent:eventClass id:eventID parameters:c[0], v[0], c[1], v[1], c[2], v[2], end];
                break;
            case 4:
                value = [object sendEvent:eventClass id:eventID
                               parameters:c[0], v[0], c[1], v[1], c[2], v[2], c[3], v[3], end];
                break;
            case 5:
                value = [object sendEvent:eventClass id:eventID
                               parameters:c[0], v[0], c[1], v[1], c[2], v[2], c[3], v[3], c[4], v[4], end];
                break;
            case 6:
                value = [object sendEvent:eventClass id:eventID
                               parameters:c[0], v[0], c[1], v[1], c[2], v[2], c[3], v[3], c[4], v[4], c[5], v[5], end];
                break;
            case 7:
                value = [object sendEvent:eventClass id:eventID
                               parameters:c[0], v[0], c[1], v[1], c[2], v[2], c[3], v[3], c[4], v[4], c[5], v[5],
                                          c[6], v[6], end];
                break;
            default:
                value = [object sendEvent:eventClass id:eventID
                               parameters:c[0], v[0], c[1], v[1], c[2], v[2], c[3], v[3], c[4], v[4], c[5], v[5],
                                          c[6], v[6], c[7], v[7], end];
                break;
        }
        *result = value;
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}

BOOL SBRSPropertyWithCode(
    id target,
    AEKeyword code,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        if (![target isKindOfClass:[SBObject class]]) {
            *message = [NSString stringWithFormat:@"%@ is not an SBObject", [target class]];
            return NO;
        }
        *result = [(SBObject *)target propertyWithCode:code];
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}

BOOL SBRSPropertyWithClassAndCode(
    id target,
    Class scriptingClass,
    AEKeyword code,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        if (![target isKindOfClass:[SBObject class]]) {
            *message = [NSString stringWithFormat:@"%@ is not an SBObject", [target class]];
            return NO;
        }
        *result = [(SBObject *)target propertyWithClass:scriptingClass code:code];
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}

BOOL SBRSElementArrayWithCode(
    id target,
    AEKeyword code,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message) {
    @try {
        if (![target isKindOfClass:[SBObject class]]) {
            *message = [NSString stringWithFormat:@"%@ is not an SBObject", [target class]];
            return NO;
        }
        *result = [(SBObject *)target elementArrayWithCode:code];
        return YES;
    } @catch (id exception) {
        *message = SBRSExceptionMessage(exception);
        return NO;
    }
}
