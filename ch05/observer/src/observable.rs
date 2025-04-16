// 可被观察的 trait
// 一个可被观察的对象通常意义上就是一个消息队列
// 该 trait 在内部保存 3 种东西
// 第一个是 订阅了本消息队列的观察者 队列
// 第二个是本消息队列的状态发生改变（事件发生）时的通知方法，
//   以调用其回调函数的形式通知保存在队列里的所有观察者
// 第三个是入队及出队方法。
pub trait Observable {
    type Observer;
    fn update(&self);
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
}
