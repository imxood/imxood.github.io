# Rust 笔记

## Pin

参考: https://folyd.com/blog/rust-pin-unpin/

Pin自身是一个智能指针. 为什么呢? 因为他impl了Deref和DerefMut. 
Pin包裹的内容只能是指针, 不能是其他普通类型. 比如Pin<u32>就没有意义. 

Pin具有"钉住"T不能移动的功能, 这个功能是否生效取决于T是否impl Unpin. 简单的说, 如果T实现了Unpin, Pin的"钉住"功能完全失效了, 这时候的Pin<P<T>>就等价于P<T>. 

Unpin是一个auto trait, 编译器默认会给所有类型实现Unpin. 唯独有几个例外, 他们实现的是!Unpin. 这几个例外是PhantomPinned, 编译器为async/await desugar之后生成的impl Future的结构体. 

所以Pin<P<T>>默认情况下的"钉住"功能是不生效的

```rust
pub struct Pin<P> {
    pointer: P,
}

#[stable(feature = "pin", since = "1.33.0")]
impl<P: Deref> Deref for Pin<P> {
    ...
}

#[stable(feature = "pin", since = "1.33.0")]
impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> {
    ...
}
```

PhantomPinned, 实现了 !Unpin

```rust
struct Test {
   a: String,
   b: *const String,
   _marker: PhantomPinned,
}
```

### Pin<P<T>>

如果P<T>符合Unpin, 那P<T>从被Pin包裹到被销毁, 都要一直保证P<T>不被钉住
如果P<T>符合!Unpin, 那P<T>从被Pin包裹到被销毁, 都要一直保证P<T>被钉住