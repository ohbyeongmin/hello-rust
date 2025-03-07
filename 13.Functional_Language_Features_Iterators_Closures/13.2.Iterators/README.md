# Processing a Series of Items with Iterators

Iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

When the `for` loop is called using the iterator, each element in the iterator is used in one iteration of the loop, which prints out each value.

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
```

## The `Iterator` Trait and the `next` Method

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

`Iterator` trate requires `Item` type, and `Item` type will be the type returned from the iterator.

```rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

`v1_iter` must be mutable. Calling the `next` method on an iteraotr changes internal state that the iterator uses to keep track of where it is in the sequence.

The values we get from the calls to `next` are immutable references to the values in the vector.

The `iter` method produces an iterator over immutable references.

If we want to get ownership of `v1` and return owned values. we can call `into_iter`.

If we want to iterate over mutalbe references, we can call `iter_mut`.

## Methods that Consume the Iterator

Methods that call `next` are called _consuming adapters_, because calling them uses up the iterator.

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

`sum` takes ownership of the iterator.

## Methods that Produce Other Iterators

_Iterator adapters_ are methods defined on the `Iterator` trait don't consume the iterator.

Instead, they produce different iterators by changing some aspect of the original iterator.

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```

This code produces a warning: Iterator adapters are lazy, and we need to consume the iterator here.

To fix this warning and consume the iterator, we'll use the `collect` method.
This method consumes the iterator and collects the resulting values into a collection data type.

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```

## Using Closures that Capture Their Environment

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```
