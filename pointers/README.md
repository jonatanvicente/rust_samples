
### Quick Comparison Table of Rust Pointer Types

| Pointer Type  | Ownership  |   Thread Safe?   |     Mutability      |
|:-------------:|:----------:|:----------------:|:-------------------:|
|      **&T**       |  Borrowed  | Yes (Read-only)  |      Immutable      |
|    **&mut T**     |  Borrowed  |        No        |       Mutable       |
|    **Box<T>**     |   Owned    |       Yes        |       Mutable       |
|     **Rc<T>**     |   Shared   |        No        |      Immutable      |
|    **Arc<T>**     |   Shared   |       Yes        |      Immutable      |
|   ***const T**    |    None    |        No        | Immutable (Unsafe)  |