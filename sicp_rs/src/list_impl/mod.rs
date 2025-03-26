// src/list/mod.rs
//! # List Implementation
//!
//! This module defines a `List` type that supports recursive and nested list structures,
//! inspired by the Lisp-style linked lists commonly seen in functional programming.
//! It provides various utility methods for manipulating lists, such as mapping, filtering,
//! folding, and more. Additionally, it includes macros for creating lists and wrapping values.
//!
//! ## Features
//! - Nested list structures (`List::Cons`).
//! - Immutable value wrapping (`List::V`).
//! - Utility methods for list manipulation, such as `map`, `filter`, `fold_left`, and more.
//! - Support for comparing lists and values (`PartialEq` and `PartialOrd`).
//!
//! ## Examples
//!
//! ### Creating a List
//! ```rust
//! use sicp_rs::prelude::*;
//!
//! // Create a nested list
//! let nested_list = list![1, "hello", list![2, 3]];
//! println!("{}", nested_list); // Output: (1, ("hello", ((2, (3, Nil)), Nil)))
//!
//! // Manipulate the list
//! let reversed = nested_list.deep_reverse();
//! println!("{}", reversed); // Output: (((3, (2, Nil)), Nil), ("hello", (1, Nil)))
//! ```
//!
//! ### Working with Values
//! ```rust
//! use sicp_rs::prelude::*;
//!
//! let list = list![1, 2, 3];
//! assert_eq!(list.length(), 3);
//! let mapped = list.map(|x| (x.try_as_basis_value::<i32>().unwrap() * 2).to_listv());
//! assert_eq!(mapped.to_string(), "(2, (4, (6, Nil)))");
//! ```
//!
//! ### Filtering and Folding
//! ```rust
//! use sicp_rs::prelude::*;
//!
//! let list = list![1, 2, 3, 4, 5_i32];
//!
//! // Filter even numbers
//! let filtered = list.filter(|x| x.try_as_basis_value::<i32>().unwrap() % 2 == 0);
//! assert_eq!(filtered.to_string(), "(2, (4, Nil))");
//!
//! // Sum up all elements
//! let sum = list.fold_left(|acc, x| acc + x.try_as_basis_value::<i32>().unwrap(), 0);
//! assert_eq!(sum, 15);
//! ```

use crate::prelude::ListV;
use std::any::TypeId;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
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
#[derive(Debug)]
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
    /// use sicp_rs::prelude::List;
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
    pub fn last_pair(&self) -> List {
        if self.tail().is_empty() {
            self.clone()
        } else {
            self.tail().last_pair()
        }
    }
    /// Wrap a value as `List::V`.
    /// 将基础值封装为 `List::V`。
    pub fn wrap_as_list_value<T: ListV>(v: T) -> List {
        List::V(Rc::new(v))
    }
    /// 非基础值无需封装，直接返回自身
    pub fn to_listv(self) -> List {
        self
    }
    pub fn get_basis_value(&self) -> Rc<dyn ListV> {
        match self {
            List::V(v) => v.clone(),
            _ => unreachable_with_location("Only List::V(_) can call get_basis_value", &self),
        }
    }
    pub fn is_number_value(&self) -> bool {
        self.is_value() && self.get_basis_value().as_ref().is_number()
    }
    pub fn is_string_value(&self) -> bool {
        self.is_value() && self.get_basis_value().as_ref().is_string()
    }
    pub fn is_float_value(&self) -> bool {
        self.is_value() && self.get_basis_value().as_ref().is_float()
    }
    pub fn is_integer_value(&self) -> bool {
        self.is_value() && self.get_basis_value().as_ref().is_integer()
    }
    pub fn try_as_basis_value<T: Clone + std::fmt::Debug + 'static>(
        &self,
    ) -> Result<&T, &'static str> {
        match self {
            List::V(v) => {
                // 必须首先as_ref()从Rc中解出dyn ListV,才是正确的Any类型,从而正确解出类型T的值
                let any = v.as_ref().as_any();

                if let Some(value) = any.downcast_ref::<T>() {
                    Ok(value)
                } else {
                    Err("Type mismatch")
                }
            }
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
    /// use sicp_rs::prelude::*;
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
    pub fn append_mutator(&self, y: List) -> List {
        assert!(!self.is_empty(), "append_mutator cannot work on empty pair");
        self.last_pair().set_tail(y);
        self.clone()
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
            List::V(_) => {
                //unreachable_with_location("Accumulate only accept list", &self),
                eprintln!(
                    "Warning: accumulate only accepts list, not value. May caused by pair construct not ending with Nil."
                );
                fun(self, initial)
            }
            
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
            .accumulate(|current, result| current.append(&result), List::Nil)
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
                List::V(_) => {
                   // unreachable_with_location("Flod_left only accept list", &rest)},
                    eprintln!(
                        "Warning: fold_left only accepts list, not value. May caused by pair construct not ending with Nil."
                    );
                    op(result, &rest)
                }
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
            List::V(_) => {
                //unreachable_with_location("Accumulate_n only accept list", &self)
                panic!(
                    "Error: accumulate_n only accepts list, not value. May caused by pair construct not ending with Nil."
                );
            },
          
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
    pub fn find_index(&self, x: &List) -> Option<usize> {
        if self.head().is_empty() {
            None
        } else if self.head() == *x {
            Some(0)
        } else {
            let n = self.tail().find_index(x);
            if let Some(n) = n { Some(n + 1) } else { None }
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
    pub fn pretty_print(&self) -> String {
        format!(
            "({}",
            self.accumulate(
                |current, result| {
                    let print_str = if current.is_empty() {
                        // accumulate never return empty list, do nothing
                        "".to_string()
                    } else if current.is_pair() {
                        current.pretty_print()
                    } else {
                        current.to_string()
                    };
                    //println!("current value:{} print_str {}", current, print_str);
                    if result.starts_with(")") || result == "" {
                        format!("{}{}", print_str, result)
                    } else {
                        format!("{}, {}", print_str, result)
                    }
                },
                ")".to_string(),
            )
        )
    }
}

#[inline(always)]
pub fn unreachable_with_location(message: &str, self_repr: &impl std::fmt::Display) -> ! {
    unreachable!(
        "{}, Found {}. Called from {}:{}",
        message,
        self_repr,
        std::panic::Location::caller().file(),
        std::panic::Location::caller().line()
    );
}
#[inline(always)]
pub fn panic_with_location(message: &str, self_repr: &impl std::fmt::Display) -> ! {
    panic!(
        "{}, Found {}. Called from {}:{}",
        message,
        self_repr,
        std::panic::Location::caller().file(),
        std::panic::Location::caller().line()
    );
}
pub fn apply_in_underlying_rust(prim: impl Fn(&List) -> List, arglist: &List) -> List {
    prim(arglist)
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
                write!(f, "{}", t.as_ref().as_string())
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

/// Macro for creating a list from values.
/// 用于从多个值创建链表的宏。
#[macro_export]
macro_rules! list {
    () => {
        $crate::list_impl::List::Nil
    };
    ($($val:expr),+ $(,)?) => {
        $crate::list_impl::List::from_slice(&[
            $($val.to_listv()),*
        ])
    };
}

/// Macro for creating a pair of two lists.
/// 用于创建两个链表的配对。
#[macro_export]
macro_rules! pair {
    ($a:expr, $b:expr) => {
        $crate::list_impl::List::pair($a.to_listv(), $b.to_listv())
    };
}

/// 用于包装闭包类型，实现 Debug+Clone trait & 类型擦除，从而支持List存储与取出并解析值
pub struct ClosureWrapper {
    func: Rc<dyn Fn(&List) -> Option<List>>,
}

impl ClosureWrapper {
    // 创建一个新的 ClosureWrapper
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(&List) -> Option<List> + 'static,
    {
        ClosureWrapper {
            func: Rc::new(func),
        }
    }

    // 调用存储的闭包
    pub fn call(&self, args: &List) -> Option<List> {
        (self.func)(args)
    }
}

// 实现 Clone
impl Clone for ClosureWrapper {
    fn clone(&self) -> Self {
        ClosureWrapper {
            func: Rc::clone(&self.func),
        }
    }
}

// 实现 Debug
impl fmt::Debug for ClosureWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A closure wrapped in ClosureWrapper")
    }
}
