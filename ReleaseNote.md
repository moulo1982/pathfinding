# Release Note

`0.0.1` 初始版本，划分了文件结构

`0.0.2` 利用RefCell和Rc来解决点引用的问题。

`0.0.3` 发现trait的结构不正确，应该为：Map是一个trait，AStart或者Recast来实现它。

`0.0.4` PartialEq 这个trait原来也可以用来比较两个不同类型之间的比较，但是A == B 和 B == A需要实现两遍

`0.0.5` 2023.12月25日：尝试tokio以 多线程+await 的模式来运行，MapManager是一个单例，new_astar需要写锁，但是find_path只需要读锁。
当中试图用到了Rust 1.75的新特性，也就是：trait中可以有async方法，这个是个很重要的修改。 

Rust 1.75 两个重大的修改就是：
1：trait支持async方法
2：返回类型支持 impl Trait

Rust团队承诺2023.12.28发布1.75 [Rust 1.75.0](https://releases.rs/docs/1.75.0/)

因为tokio的mutex.lock是await的，所以有些方法的签名必须是async，
等1.75发布就修改trait Map的find_path为async


`0.0.6` 优化一些不必要的锁，和一些返回类型

`0.0.7` 优化MapManager单例获取的方式

`0.0.8` 优化IdGenerator单例的获取方式，简化id生成的规则：（64位时间戳+64位序号）

`0.0.9` 实现A*算法，但是比较粗糙，16 * 16格子，从(1，0)寻到(14，15)，循环1000次，用时100ms左右

`0.0.10` 重大优化，将openList中存储点的数据结构改为BTreeMap<i64, Vec < PointType > > 
        其中key代表A*算法中的F，这样快速取出F最小的点只要first_entry就可以，然后从Vec中pop最后一个即可。
        查询一个点在BTreeMap是否存在，通过F取出vec后，循环判断。如果要删除就从vec中删除。

        example中：
        使用256*256格子，从(1，0)寻到(254，255)，单次，Debug 50ms左右，Release 33ms左右
        按理说，1000次应该在33000左右，再除以内核数16，应该在2000ms（预估）左右，因为Intel的超线程技术
        16和有24个逻辑线程，最终执行结果为下面的1150ms左右。所以超线程技术还是有用的。提升极大。

        优化前：
        16*16格子，从(1，0)寻到(14，15)，循环1000次，用时100ms左右

        优化后：
        16*16格子，   从(1，0)寻到(14，15)，循环1000次，用时（20ms）左右.
        32*32格子，   从(1，0)寻到(30，31)，循环1000次，用时（50ms）左右.
        64*64格子，   从(1，0)寻到(63，63)，循环1000次，用时（120ms）左右.
        128*128格子， 从(1，0)寻到(126，127)，循环1000次，用时（330ms）左右.
        256*256格子， 从(1，0)寻到(254，255)，循环1000次，用时（1100ms）左右.

        开发机：
        CPU：
                13th Gen Intel(R) Core(TM) i7-13700F
                内核:	16 （8小核+8大核）
                逻辑处理器:	24（只有8个大核有超线程技术，所以是24逻辑核）
        内存：
                64GB DDR4 4800 MHz

        寻路使用tokio异步，默认线程数。

