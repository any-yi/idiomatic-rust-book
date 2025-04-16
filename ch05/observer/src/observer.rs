// 观察者 trait
// 该 trait 保存两个东西
// 一个是感兴趣的主题（可以理解为一个消息队列），
// 另一个是当感兴趣的主题消息发生时需要调用的代码（回调函数），
//   该回调函数被传入了主题，因此可以通过这个传入参数读取主题状态
pub trait Observer {
    type Subject;
    fn observe(&self, subject: &Self::Subject);
}
