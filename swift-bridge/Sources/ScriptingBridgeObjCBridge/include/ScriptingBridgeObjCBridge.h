#import <CoreServices/CoreServices.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

typedef NS_ENUM(NSInteger, SBRSReturnKind) {
    SBRSReturnKindObject = 0,
    SBRSReturnKindVoid = 1,
    SBRSReturnKindObjectOrVoid = 2,
};

BOOL SBRSInvoke(
    id target,
    NSString *selectorName,
    id _Nullable argument,
    BOOL hasArgument,
    SBRSReturnKind returnKind,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

BOOL SBRSValueForKeyPath(
    id target,
    NSString *keyPath,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

BOOL SBRSArrayByApplyingSelector(
    id array,
    NSString *selectorName,
    id _Nullable argument,
    BOOL hasArgument,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

BOOL SBRSSendEvent(
    id target,
    AEEventClass eventClass,
    AEEventID eventID,
    NSArray<NSNumber *> *codes,
    NSArray *values,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

BOOL SBRSPropertyWithCode(
    id target,
    AEKeyword code,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

BOOL SBRSPropertyWithClassAndCode(
    id target,
    Class scriptingClass,
    AEKeyword code,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

BOOL SBRSElementArrayWithCode(
    id target,
    AEKeyword code,
    id _Nullable * _Nonnull result,
    NSString * _Nullable * _Nonnull message);

NS_ASSUME_NONNULL_END
