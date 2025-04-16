use std::sync::{Arc, Weak};

use crate::{observable::Observable, observer::Observer};

// subject 有主题、起因的意思
pub struct Subject {
    // subject 里面保存了观察者队列
    observers: Vec<Weak<dyn Observer<Subject = Self>>>,
    state: String,
}

impl Subject {
    pub fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into(),
        }
    }

    pub fn state(&self) -> &str {
        self.state.as_ref()
    }
}

// 使 subject 可被观察
impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;
    // 调用每个观察者的 observe 方法，通知被观察的 subject 状态已改变
    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self)); // 注意 observe
                                            // 方法被传入了主题，为了让观察者能够读取主题的状态
    }
    // 观察者入队
    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }
    // 观察者出队
    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(|f| !f.ptr_eq(&Arc::downgrade(&observer)));
    }
}
