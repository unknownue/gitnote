---
tags: [Rust]
title: Different between associate type and generic type
created: '2019-12-27T12:13:30.314Z'
modified: '2019-12-27T12:14:13.420Z'
---

# Different between associate type and generic type

> from https://rust.cc/article?id=1a2e348e-a4d0-4ee1-9368-353730f2e212

关联类型是 trait 定义中的类型占位符。定义的时候，并不定义它的具体的类型是什么。在 impl 这个 trait 的时候，才为这个关联类型赋予确定的类型。也就是说，在实现的时候，才知道它的具体类型是什么。其实使用泛型也可以做到类似的效果。

trait 中的泛型与关联类型，有如下区别：

- 如果 trait 中包含泛型参数，那么，可以对同一个目标类型，多次 impl 此 trait，每次提供不同的泛型参数。而关联类型方式只允许对目标类型实现一次。
- 如果 trait 中包含泛型参数，那么在具体方法调用的时候，必须加以类型标注以明确使用的是哪一个具体的实现。而关联类型方式具体调用时不需要标注类型（因为不存在模棱两可的情况）。
