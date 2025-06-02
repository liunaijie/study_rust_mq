配置一个多模块`workspace`Rust项目的样例

# 模块说明
在这个样例中,总共有4个模块(我仍然以Java中的module模块来这样称呼它)。
分别为:
- cmd (入口模块,对应Rust中的lib)
- common-base (抽离出来的公共代码,对应Rust中的lib)
- placement-center (某个业务模块的代码,对应Rust中的lib)
- protocol (某个业务模块的代码,对应Rust中的lib)

这里的演示代码很简单,在`common-base`中定义个公共方法,由业务模块调用, 再由入口类调用.

# 具体配置
## 父Cargo

```toml
# 主要配置,定义有哪些子模块,文件位置
[workspace]
members = [
    "src/cmd",
    "src/common-base",
    "src/placement-center",
    "src/protocol"
]

# 这里是为了统一每个子模块的版本,每个子模块的版本使用这里的变量值
# 如果某个子模块的版本需要单独管理,也可以单独设置
[workspace.package]
version = "0.1.0"
edition = "2024"
```
## common-base模块
### Cargo设置
```toml
# 设置自己模块的名称,以及version,edition都需要使用父模块的值
[package]
name = "common-base"
version.workspace = true
edition.workspace = true
# version = "0.2.0"  # 也可以单独设置版本号

[dependencies]
```
### 代码访问权限
由于`common-base`是一个`crate`,所以我们需要在`src`下创建一个`lib.rs`  
并且我想将公共的代码拆分到不同的类里面,不同的文件夹下面.  
这里展示一下如何进行这两种方式的代码编写和配置:
```
├── config_util
│   ├── config_util.rs
│   └── mod.rs
├── protocol_util
│   └── mod.rs
└── lib.rs    
```
`config_util`文件夹与`protocol_util`文件夹在Rust中都是两个`mod(module)`,所以都需要一个`mod.rs`文件.  
在`protocol_util`相关的代码中,我直接将代码放到了`mod.rs`中, 没有再继续拆分  
在`config_util`中,我将代码的实现放到了一个单独的类文件中, 这相当于在`mod`的层级关系上又添加了一层.  
`config_util.rs`的代码定义我们需要在`config_util/mod.rs`中暴露出去.
```rust
// common-base/src/config_util/mod.rs
pub mod config_util; // 设置为pub,从而可以让其他crate饮用
```  

具体的方法实现上, 我们需要添加`pub`关键字,从而让这个方法被其他类进行调用.  

最后,我们需要在`lib.rs`中将这两个`mod(module)`再次暴露出去
```rust
pub mod config_util;
pub mod protocol_util;
```

到此,我们就完成了`common-base`模块的代码编写和权限设置, 其他的业务模块都可以调用这里的代码实现了.

## placement-center模块
这个模块中, 我们需要调用`common-base`中的公共代码, 首先要导入依赖
```toml
[package]
name = "placement-center"
version.workspace = true
edition.workspace = true

[dependencies]
common-base = {path = "../common-base"} # 导入本地依赖

# serde_json = "1.0" # 通过这种方式导入其他的依赖
```

这里的代码实现就很简单, 在`lib.rs`中定义一个函数,这个函数再调用`common-base`的函数
```rust
// 这里的导入层级为: crate_name::mod_name:mod_name::class_file_name
use common_base::config_util::config_util::config_util;

pub fn update_config() -> String {
    config_util("aaa")
}
```

## cmd模块
### Cargo
```toml
[package]
name = "cmd"
version.workspace = true
edition.workspace = true


[dependencies]
# 导入依赖 需要写在bin前面
# 我们需要导入两个业务模块的module
placement-center = {path = "../placement-center"}
protocol = {path = "../protocol"}

# bin表示主入口函数的文件,用于指定程序启动时运行的程序
# name表示编译生产的二进制文件的名称
[[bin]]
name = "placement-center"
path = "src/placement-center/server.rs"
# 我们也可以指定多个bin,生成多个目标二进制文件
#[[bin]]
#name = "placement-center"
#path = "src/placement-center/server.rs"
#placement-center = {path = "../common-base" }

```

### 代码调用
```rust
use placement_center::update_config;
use protocol::protocol_fnc;

fn main() {
    println!("Get Started");
    // 这里就直接进行函数调用即可
    println!("{}", protocol_fnc(100000));
    println!("{}", protocol_fnc(2));
    println!("{}", update_config());
}

```


# 打包调用
我们在父层目录直接运行`cargo build`后可以在`target/debug`文件夹下找到一个`placement-center`的文件.   
运行命令`./target/debug/placement-center`后就可以运行我们上面的这段代码了.  
名称为什么是`placement-center`是因为我们在`cmd`模块的`Cargo`中定义了
```toml
[[bin]]
name = "placement-center"
path = "src/placement-center/server.rs"
```
所以会产生一个名为`placement-center`的可运行文件,并且入口函数为`server.rs`.  
你可以尝试一下将名称修改或者启动类修改一下,或者同时定义多个入口类,来看下最终的结果.

我们现在打包完成后,文件存放在`debug`文件夹下, 这是因为`cargo build`默认执行的是`dev`的Profile.  
我们可以运行`cargo build --release`来进行生产环境的打包.  
这两个的区别是`dev`模式会关注与编译速度, 对于运行速度不会特别优化, 这对于我们日常开发来说是足够的.   
`release`模式则是关注与运行速度, 从而编译时间会较长, 我们在线上环境应该使用这个模式.  

# 优化打包
通过自定义`makefile`来设置我们项目的最终打包格式,以使得项目最终打包后可以与很多开源软件包的结构相似
```
.
├── bin  # 存放启动脚本
├── config # 存放默认配置
└── libs # 存放依赖的可执行文件
```

```makefile
TARGET = multiple-crate
BUILD_DIR = ./build
VERSION := $(shell grep '^version = ' Cargo.toml | head -n1 | cut -d'"' -f2)
PACKAGE_FOLD_NAME = $(TARGET)-$(VERSION)

release:
	@echo "Building $(PACKAGE_FOLD_NAME)..."
	# 创建对应目录
	mkdir -p $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)
	mkdir -p $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/bin
	mkdir -p $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/libs
	mkdir -p $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/config
	# 编译 release 包
	cargo build --release

	# 拷贝 bin目录下的脚本、config中的配置文件、编译成功的可执行文件
	cp -rf target/release/placement-center $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/libs
	cp -rf bin/* $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/bin
	cp -rf config/* $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/config
	chmod -R 777 $(BUILD_DIR)/$(PACKAGE_FOLD_NAME)/bin/*
	
	# 将目录打包成.tar.gz 文件
	cd $(BUILD_DIR) && tar zcvf $(PACKAGE_FOLD_NAME).tar.gz $(PACKAGE_FOLD_NAME) && rm -rf $(PACKAGE_FOLD_NAME)
	@echo "build release package success. $(PACKAGE_FOLD_NAME).tar.gz"

clean:
	cargo clean
	rm -rf $(BUILD_DIR)

```
定义这个文件时,需要注意文件的缩进, Makefile要求使用Tab而不是空格


