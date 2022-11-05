# TomatoOS
跟个西红柿的系统
# 环境配置
需要你自己去配置好Rust的环境  
配置好后运行make fixbuild  
运行完成后记得把Cargo.toml中的dependencies下的xbuild和bootimage删掉

# 构建
直接运行make  
运行完成后需要用QEMU运行编译完成的bin文件  
例如`qemu-system-x86_64 -drive format=raw,file=bootimage-tomato_os.bin`