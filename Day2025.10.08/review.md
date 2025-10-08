# 学习与反思以及碎碎念
昨天完成了rustlings后，完成了基本的学习后，我开始按照教程做一个mini-web-server，第一步上来就是写一个简易的线程池，这很有意思。
## 知识点1：Mutex获取锁的过程是有可能失败的。
如果其他获取了锁的线程panic了，那么去获取这个Mutex的时候，就会返回失败，所以在获取锁的时候，也要对锁进行处理（如unwrap）。
## 知识点2：Mutex的详细上锁解锁时间点
Mutex的上锁时间点为调用lock()内部的原子上锁指令的那一刻开始
而Mutex的解锁时间点为Mutex被drop的那一刻，就直接解锁了，可以不显式调用（RAII保证当Mutex超出作用域之后，就会隐式调用drop），如果想要提前释放锁，那么可以显示调用drop来释放锁。需要注意的是，移动锁的所有权是不会drop的，所以不会释放锁。
## 知识点3：Option变量移动所有权
这个场景主要用于，你有一个Option变量，你希望移动变量里面的那个内容的所有权，在原变量这里留下一个None，而不是直接将整个Option变量移走，导致整个变量无法访问。
可以使用Option.take方法，以下是该方法的签名：
```Rust
pub fn take(&mut self) -> Option<T>
```
显而易见的，如果原Option变量是Some类型，那么会直接返回一个Some类型，并将原Option变量置为None。
而如果原Option变量是None类型的，那么会直接返回None类型。
## 知识点4：如果对一个Option变量drop
当这个Option变量是Some类型时，会先调用里面的值的drop函数，再调用Some本身的drop进行资源回收工作。
如果是None类型，就不用说了，直接drop
## 知识点4：let job = receiver.lock().unwrap().recv().unwrap();的作用域
这很有意思，我是在锈书里面看到的这个问题，这里的**receiver**是一个被**Mutex包住的发送者**，那么这个Mutex会在哪里释放？
**答案是就在这个表达式结束。**
我以为会是下一个大括号这类的地方，但为什么只到这个表达式结束？请看**lock()的函数签名**：
```Rust
pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
```
从签名中我们可以看到，这里会**直接返回锁本身**，而在调用了receiver.lock()后，我们**没有用另外一个变量去承载这个锁**，就直接继续在后面调用了这个recv()，也就是说这个Mutex锁其实是用一个**临时变量**存储的。在Rust中，一个表达式内用的临时变量，全部都会在这个表达式完成后释放。
如果想要达成我之前预想的到下一个大括号再drop，则需要把这个锁通过一个正式变量承载：
```Rust
// 伪执行顺序
let job = {
    let guard = receiver.lock().unwrap(); // ① 上锁
    let tmp   = guard.recv().unwrap();    // ② 用锁里的 channel
    tmp                                   // ③ 返回值
};                                        // ④ guard 被 drop → 解锁
```
## 碎碎念时间
今天完成了多线程的简单web服务器的小项目，虽然是小项目，但我在实现简单的线程池的过程中，依旧遇到很多新的问题和新的思考。
明天开始学习据说的Rust最强异步运行时tokio，冲刺！
但是明天要上班，sad