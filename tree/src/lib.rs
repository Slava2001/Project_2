use std::{
    cell::{Ref, RefCell, RefMut},
    mem::swap,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct InnerNode<T> {
    data: T,
    childs: Vec<Rc<RefCell<InnerNode<T>>>>,
    parent: Option<Weak<RefCell<InnerNode<T>>>>,
}
#[derive(Debug)]
pub struct Node<T> {
    node: Rc<RefCell<InnerNode<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { node: Rc::new(RefCell::new(InnerNode { data, childs: Vec::new(), parent: None })) }
    }

    pub fn add_child(&mut self, data: T) -> Node<T> {
        let node = Self::new(data);
        self.node.borrow_mut().childs.push(node.node.clone());
        node.node.clone().borrow_mut().parent = Some(Rc::downgrade(&self.node));
        node
    }

    pub fn borrow<'a>(&'a self) -> DataRef<'a, T> {
        DataRef { node: self.node.borrow() }
    }

    pub fn borrow_mut<'a>(&'a self) -> DataRefMut<'a, T> {
        DataRefMut { node: self.node.borrow_mut() }
    }

    pub fn detach(&mut self) -> Option<Node<T>> {
        let mut parent = None;
        swap(&mut self.node.borrow_mut().parent, &mut parent);
        if let Some(parent) = parent {
            if let Some(parent) = parent.upgrade() {
                parent.borrow_mut().childs.retain(|v| !Rc::ptr_eq(v, &self.node));
                return Some(Node{node: parent});
            }
        }
        None
    }

    pub fn attach(&mut self, mut child: Node<T>) -> Node<T> {
        child.detach();
        self.node.borrow_mut().childs.push(child.node.clone());
        child.node.borrow_mut().parent = Some(Rc::downgrade(&self.node));
        child
    }
}

pub struct DataRef<'a, T> {
    node: Ref<'a, InnerNode<T>>,
}

impl<'a, T> Deref for DataRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node.data
    }
}

pub struct DataRefMut<'a, T> {
    node: RefMut<'a, InnerNode<T>>,
}

impl<'a, T> Deref for DataRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node.data
    }
}

impl<'a, T> DerefMut for DataRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node.data
    }
}
