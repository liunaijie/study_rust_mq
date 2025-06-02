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
