//! The names of TypeScript's lib types a JSDoc type may use without declaring them (the standard
//! library's type-only globals, and the utility types).

pub const TS_LIB_TYPES: &[&str] = &[
    "AggregateErrorConstructor", "ArrayBufferConstructor", "ArrayBufferLike", "ArrayBufferTypes", "ArrayBufferView", "ArrayConstructor",
    "ArrayIterator", "ArrayLike", "AsyncDisposable", "AsyncDisposableStackConstructor", "AsyncGenerator", "AsyncGeneratorFunction",
    "AsyncGeneratorFunctionConstructor", "AsyncIterable", "AsyncIterableIterator", "AsyncIterator", "AsyncIteratorObject", "Awaited",
    "BigInt64ArrayConstructor", "BigIntConstructor", "BigIntToLocaleStringOptions", "BigUint64ArrayConstructor", "BooleanConstructor",
    "BuiltinIteratorReturn", "CallableFunction", "Capitalize", "ClassAccessorDecoratorContext", "ClassAccessorDecoratorResult",
    "ClassAccessorDecoratorTarget", "ClassDecorator", "ClassDecoratorContext", "ClassFieldDecoratorContext", "ClassGetterDecoratorContext",
    "ClassMemberDecoratorContext", "ClassMethodDecoratorContext", "ClassSetterDecoratorContext", "ConcatArray", "ConstructorParameters",
    "DataViewConstructor", "DateConstructor", "DecoratorContext", "DecoratorMetadata", "DecoratorMetadataObject", "Disposable",
    "DisposableStackConstructor", "ErrorConstructor", "ErrorOptions", "EvalErrorConstructor", "Exclude", "Extract", "FinalizationRegistryConstructor",
    "FlatArray", "Float32ArrayConstructor", "Float64ArrayConstructor", "FunctionConstructor", "Generator", "GeneratorFunction",
    "GeneratorFunctionConstructor", "IArguments", "ImportAssertions", "ImportAttributes", "ImportCallOptions", "ImportMeta", "InstanceType",
    "Int16ArrayConstructor", "Int32ArrayConstructor", "Int8ArrayConstructor", "IterableIterator", "IteratorObjectConstructor", "IteratorResult",
    "IteratorReturnResult", "IteratorYieldResult", "Lowercase", "MapConstructor", "MapIterator", "MethodDecorator", "NewableFunction", "NoInfer",
    "NonNullable", "NumberConstructor", "ObjectConstructor", "Omit", "OmitThisParameter", "ParameterDecorator", "Parameters", "Partial", "Pick",
    "PromiseConstructor", "PromiseConstructorLike", "PromiseFulfilledResult", "PromiseLike", "PromiseRejectedResult", "PromiseSettledResult",
    "PromiseWithResolvers", "PropertyDecorator", "PropertyDescriptor", "PropertyDescriptorMap", "PropertyKey", "ProxyConstructor", "ProxyHandler",
    "RangeErrorConstructor", "Readonly", "ReadonlyArray", "ReadonlyMap", "ReadonlySet", "ReadonlySetLike", "Record", "ReferenceErrorConstructor",
    "RegExpConstructor", "RegExpExecArray", "RegExpIndicesArray", "RegExpMatchArray", "RegExpStringIterator", "Required", "ReturnType",
    "SetConstructor", "SetIterator", "SharedArrayBufferConstructor", "StringConstructor", "StringIterator", "SuppressedErrorConstructor",
    "SymbolConstructor", "SyntaxErrorConstructor", "TemplateStringsArray", "ThisParameterType", "ThisType", "TypeErrorConstructor",
    "TypedPropertyDescriptor", "URIErrorConstructor", "Uint16ArrayConstructor", "Uint32ArrayConstructor", "Uint8ArrayConstructor",
    "Uint8ClampedArrayConstructor", "Uncapitalize", "Uppercase", "WeakKey", "WeakKeyTypes", "WeakMapConstructor", "WeakRefConstructor",
    "WeakSetConstructor",
];
