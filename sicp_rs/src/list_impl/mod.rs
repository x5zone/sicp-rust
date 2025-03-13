// src/list/mod.rs
//! # List Implementation
//!
//! This module defines a `List` type that supports recursive and nested list structures.
//! It provides various utility methods for manipulating lists, such as mapping, filtering,
//! folding, and more. Additionally, it includes macros for creating lists and wrapping values.
//!
//! ## Features
//! - Nested list structures (`List::Cons`).
//! - Immutable value wrapping (`List::V`).
//! - Utility methods for list manipulation.
//! - Macros for convenient list creation.
//!
//! ## Examples
//! ```rust
//! use sicp_rs::{list, v};
//! use sicp_rs::list_impl::List;
//!
//! // Create a nested list
//! let nested_list = list![v![1, 2, 3], v![4, 5]];
//! println!("{}", nested_list); // Output: ((V(1), (V(2), (V(3), Nil))), (V(4), (V(5), Nil)))
//!
//! // Manipulate the list
//! let reversed = nested_list.deep_reverse();
//! println!("{}", reversed); // Output: ((V(5), (V(4), Nil)), ((V(3), (V(2), (V(1), Nil))), Nil))
//! ```
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::any::TypeId;
use crate::listv::ListV;
// UnsafeCell
// 预备以后的扩展,例如" 1. Sicp ch3.5中涉及并发, 可修改为 Rc->Arc; 2. 若因为借用检查导致panic RefCell->RwLock
// #[cfg(feature = "single_threaded")]
// pub type Shared<T> = Rc<T>;
// #[cfg(feature = "multi_threaded")]
// pub type Shared<T> = Arc<T>;
// #[cfg(feature = "single_threaded")]
// pub type InnerCell<T> = RefCell<T>;
// #[cfg(feature = "multi_threaded")]
// pub type InnerCell<T> = RwLock<T>;

/// Shared reference to a `List` type.
/// 使用 `Rc<RefCell<List>>` 封装的共享引用。
/// 在单线程环境下使用 `Rc`，在多线程环境下可扩展为 `Arc`。
type SharedList = Rc<RefCell<List>>;
/// Enum representing a recursive list structure.
/// 定义递归链表结构的枚举。
pub enum List {
    Cons(SharedList, SharedList),
    // The value is treated as a List type, enabling the construction of nested lists (e.g., list(1, list(1, 2))).
    // Values of type Rc<dyn ListV> are immutable; instead of modifying them, we create new values and replace the old ones.
    V(Rc<dyn ListV>),
    Nil,
}
impl Clone for List {
    fn clone(&self) -> Self {
        match self {
            List::Cons(a, b) => List::Cons((*a).clone(), (*b).clone()),
            List::V(v) => List::V(v.clone()),
            List::Nil => List::Nil,
        }
    }
}
impl List {
    /// Extract an immutable reference from a `SharedList`.
    /// 从 `SharedList` 中提取不可变引用。
    /// 
    /// # Examples
    /// ```rust
    /// use sicp_rs::list;
    /// use sicp_rs::list_impl::List;
    /// let l = list![List::Nil];
    /// let head = List::extract_clone(&l.head());
    /// assert_eq!(head.is_empty(), true);
    /// ```
    pub fn extract_clone(sl: &SharedList) -> List {
        (*sl.borrow()).clone()
    }

    /// Modify the content of a `SharedList`.
    /// 修改 `SharedList` 的内容。
    fn replace_list(old: &SharedList, new_list: List) {
        *old.borrow_mut() = new_list; 
    }
    /// Create a pair of two lists.
    /// 创建两个链表的配对。
    ///
    /// # Examples
    /// ```rust
    /// use sicp_rs::list_impl::List;
    /// let pair = List::pair(List::Nil, List::Nil);
    /// assert!(pair.is_pair());
    /// ```
    pub fn pair(a: List, b: List) -> List {
        List::Cons(Rc::new(RefCell::new(a)), Rc::new(RefCell::new(b)))
    }
    /// Check if the list is a pair.
    /// 检查链表是否是配对结构。
    pub fn is_pair(&self) -> bool {
        matches!(self, List::Cons(_, _))
    }

    /// Check if the list is empty.
    /// 检查链表是否为空。
    pub fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    /// Check if the list is a single value.
    /// 检查链表是否是单值。
    pub fn is_value(&self) -> bool {
        matches!(self, List::V(_))
    }

    /// Get the head of the list.
    /// 获取链表的头部。
    ///
    /// # Panics
    /// 如果调用者不是链表结构，将会触发 `unreachable!`。
    pub fn head(&self) -> List {
        match self {
            List::Cons(current, _) => List::extract_clone(current),
            _ => unreachable_with_location("Only list can call head", &self),
        }
    }

    /// Get the tail of the list.
    /// 获取链表的尾部。
    ///
    /// # Panics
    /// 如果调用者不是链表结构，将会触发 `unreachable!`。
    pub fn tail(&self) -> List {
        match self {
            List::Cons(_, next) => List::extract_clone(next),
            _ => unreachable_with_location("Only list can call tail", &self),
        }
    }
    /// Modify the head of the list.
    /// 修改链表的头部。
    ///
    /// # Panics
    /// 如果调用者不是链表结构，将会触发 `unreachable!`。
    pub fn set_head(&self, new_head: List) {
        match self {
            List::Cons(current, _) => List::replace_list(current, new_head),
            _ => unreachable_with_location("Only list can call set_head", &self),
        }
    }

    /// Modify the tail of the list.
    /// 修改链表的尾部。
    ///
    /// # Panics
    /// 如果调用者不是链表结构，将会触发 `unreachable!`。
    pub fn set_tail(&self, new_tail: List) {
        match self {
            List::Cons(_, next) => List::replace_list(next, new_tail),
            _ => unreachable_with_location("Only list can call set_tail", &self),
        }
    }
    /// Wrap a value as `List::V`.
    /// 将基础值封装为 `List::V`。
    ///
    /// # Examples
    /// ```rust
    /// use sicp_rs::{v, list};
    /// let value = v![42];
    /// assert!(value.is_value());
    /// ```
    pub fn wrap_as_list_value<T: ListV>(v: T) -> List {
        List::V(Rc::new(v))
    }

    pub fn get_basis_value(&self) -> Rc<dyn ListV> {
        match self {
            List::V(v) => v.clone(),
            _ => unreachable_with_location("Only List::V(_) can call get_basis_value", &self),
        }
    }

    pub fn try_as_basis_value<T: Clone + 'static>(&self) -> Result<T, &'static str> {
        match self {
            List::V(v) => v
                .as_any()
                .downcast_ref::<T>()
                .map(|value| value.clone())
                .ok_or("Type mismatch"),
            List::Nil => Err("Cannot call try_as_basis_value on List::Nil"),
            List::Cons(_, _) => Err("Cannot call try_as_basis_value on List::Cons"),
        }
    }
    pub fn get_type_id(&self) -> Option<TypeId> {
        //self.type_id()
        match self {
            List::V(v) => Some(v.as_ref().type_id()),
            _ => None,
        }
    }
    /// Create a list from a slice of `List` items.
    /// 从 `List` 的切片创建链表。
    ///
    /// # Examples
    /// ```rust
    /// use sicp_rs::list_impl::List;
    /// use sicp_rs::list;
    /// let l = list![List::Nil, List::Nil];
    /// assert_eq!(l.length(), 2);
    /// ```
    pub fn from_slice(items: &[List]) -> Self {
        items
            .iter()
            .rfold(List::Nil, |acc, item| List::pair(item.clone(), acc))
    }

    // 传入Iterator<Item = List<T>>类型,以确保既可以传入[V(1),V(2)],也可以传入[List1,List2]
    pub fn from_iterator<I: Iterator<Item = List>>(items: &mut I) -> Self {
        match items.next() {
            Some(v) => List::pair(v.clone(), List::from_iterator(items)),
            None => List::Nil,
        }
    }
    /// Append another list to the current list.
    /// 将另一个链表追加到当前链表。
    pub fn append(&self, other: &Self) -> Self {
        match self {
            List::Nil => (*other).clone(),
            List::Cons(value, next) => Self::pair(
                List::extract_clone(value),
                List::extract_clone(next).append(other),
            ),
            List::V(_) => {
                eprintln!("self is a value, not a list, convert it to list");
                Self::pair((*self).clone(), Self::Nil).append(other)
            }
        }
    }
    pub fn map<F>(&self, fun: F) -> List
    where
        F: Fn(&List) -> List,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(value, next) => List::pair(
                fun(&List::extract_clone(value)),
                List::extract_clone(next).map(fun),
            ),
            List::V(_) => fun(self),
        }
    }
    pub fn accumulate<F, U>(&self, fun: F, initial: U) -> U
    where
        F: Fn(&List, U) -> U + Clone,
    {
        match self {
            List::Nil => initial,
            List::Cons(value, next) => {
                let f = fun.clone();
                fun(
                    &List::extract_clone(value),
                    List::extract_clone(next).accumulate(f, initial),
                )
            }
            List::V(_) => unreachable_with_location("Accumulate only accept list", &self),
            //{
            //  eprintln!("accumulate only accept list, not value");
            //  fun(self, initial)},
            //}
        }
    }
    pub fn filter<F>(&self, fun: F) -> List
    where
        F: Fn(&List) -> bool,
    {
        self.accumulate(
            |current, result| {
                if fun(current) {
                    List::pair((*current).clone(), result)
                } else {
                    result
                }
            },
            List::Nil,
        )
    }
    pub fn flatmap<F>(&self, fun: F) -> List
    where
        F: Fn(&List) -> List,
    {
        self.map(fun)
            .accumulate(|current, result| result.append(&current), List::Nil)
    }
    pub fn for_each<F>(&self, fun: F) -> ()
    where
        F: Fn(&List) -> (),
    {
        match self {
            List::Nil => (),
            List::Cons(value, next) => {
                fun(&List::extract_clone(value));
                List::extract_clone(next).for_each(fun)
            }
            List::V(_) => fun(self),
        };
    }
    pub fn fold_left<U, F>(&self, fun: F, initial: U) -> U
    where
        F: Fn(U, &List) -> U + Clone,
    {
        fn iter<U, F>(op: F, result: U, rest: &List) -> U
        where
            F: Fn(U, &List) -> U + Clone,
        {
            match rest {
                List::Nil => result,
                List::Cons(value, next) => iter(
                    op.clone(),
                    op(result, &List::extract_clone(value)),
                    &List::extract_clone(next),
                ),
                List::V(_) => unreachable_with_location("Flod_left only accept list", &rest),
            }
        }
        iter(fun, initial, self)
    }
    pub fn accumulate_n<F>(&self, op: F, initial: List) -> List
    where
        F: Fn(&List, List) -> List + Clone,
    {
        match self.head() {
            List::Nil => List::Nil,
            List::Cons(_, _) => {
                let l1 = self
                    .map(|y| (y.head()).clone())
                    .accumulate(op.clone(), initial.clone());
                let l2 = self
                    .map(|y| (y.tail()).clone())
                    .accumulate_n(op.clone(), initial.clone());
                List::pair(l1, l2)
            }
            List::V(_) => unreachable_with_location("Accumulate_n only accept list", &self),
            //  eprintln!("accumulate_n only accept list, not value");
            //  fun(self, initial)},
            //}
        }
    }
    fn reverse_with<F: Fn(&List) -> List>(&self, fun: F) -> Self {
        fn reverse_with_iter<F>(l: &List, result: List, fun: F) -> List
        where
            F: Fn(&List) -> List,
        {
            match l {
                List::Nil => result,
                List::Cons(value, _) => {
                    let value = fun(&List::extract_clone(value));
                    reverse_with_iter(&l.tail(), List::pair(value, result), fun)
                }
                List::V(_) => panic!("reverse_with_iter only accept list, not value"),
            }
        }
        reverse_with_iter(self, List::Nil, fun)
    }
    pub fn reverse(&self) -> Self {
        self.reverse_with(|x| (*x).clone())
    }
    pub fn deep_reverse(&self) -> Self {
        self.reverse_with(|x| match (*x).clone() {
            List::Cons(_, _) => x.deep_reverse(),
            List::V(t) => List::V(t),
            List::Nil => List::Nil,
        })
    }
    pub fn length(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, next) => 1 + List::extract_clone(next).length(),
            List::V(_) => 1,
        }
    }
    pub fn deep_length(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(current, next) => {
                List::extract_clone(current).deep_length() + List::extract_clone(next).deep_length()
            }
            List::V(_) => 1,
        }
    }
}

#[inline(always)]
fn unreachable_with_location(message: &str, self_repr: &impl std::fmt::Display) -> ! {
    unreachable!(
        "{}, Found {}. Called from {}:{}",
        message,
        self_repr,
        std::panic::Location::caller().file(),
        std::panic::Location::caller().line()
    );
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            List::Cons(v, next) => {
                write!(
                    f,
                    "({}, {})",
                    List::extract_clone(v),
                    List::extract_clone(next)
                )
            }
            List::V(t) => {
                write!(f, "{}", t)
            }

            List::Nil => write!(f, "Nil"),
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (List::Nil, List::Nil) => true,
            (List::Cons(x1, x2), List::Cons(y1, y2)) => x1 == y1 && x2 == y2,
            (List::V(x1), List::V(y1)) => x1.as_ref() == y1.as_ref(),
            _ => false,
        }
    }
}
/// Macro for wrapping values as `List::V`.
/// 用于将值封装为 `List::V` 的宏。
#[macro_export]
macro_rules! v {
    // 单个值直接返回 `List::value`
    ($val:expr) => {
        List::wrap_as_list_value($val)
    };
    // 多个值展开为多个 `List::V`，以逗号分隔
    ($($val:expr),+ $(,)?) => {
        $(List::wrap_as_list_value($val)),*
    };
}
/// Macro for creating a list from values.
/// 用于从多个值创建链表的宏。
#[macro_export]
macro_rules! list {
    ($($val:expr),* $(,)?) => {
        List::from_slice(&[ $($val),* ])
    };
}
