# RsMaker——基于rust的免杀生成框架



## 总体分为两大模块：免杀生成与木马捆绑


### Introduction

#### 免杀生成：

 免杀目前具有三种混淆方式：1、**Aes加密** 2、**base64加密** 3、**uuid**
下图使用aes方式进行演示

<p align="center">
    <img height="400" alt="RsMaker" src="images/Jun-21-2022%2020-09-46.gif">
</p>


#### 文件捆绑：

文件捆绑支持**自定义图标**、**文件**，甚至可以将exe和exe捆绑到一起。
<p align="center">
    <img height="400" alt="RsMaker" src="images/Jun-21-2022%2020-25-47.gif">
</p>
<p align="center">
    <img height="400" alt="RsMaker" src="images/Jun-21-2022%2020-26-46.gif">
</p>

### Usage

#### 首先确保您的windows机器上安装了rust并且开启了nightly模式

```
查看rust版本
$ rustc --version
安装nightly
$ rustup install nightly
将nightly设置为默认
$ rustup default nightly
查看rust版本
$ rustc --version

```
#### 安装msvc nightly toolchain
```
rustup target add nightly-x86_64-pc-windows-msvc
```

#### 生成木马
```
rsmaker.exe generate -f beacon.bin -m aes
```

#### 捆绑文件
```
rsmaker.exe embed -f test.pdf -t loader.exe -i word.ico
```

#### 查看帮助
```
rsmaker.exe -h
```

# 注意
* 本项目所用技术仅用于学习交流，请勿直接用于任何商业场合和非法用途。
* 本项目不提供编译好的文件，仅供代码交流学习。
* 本项目为学习rust的过程中的中间产物，且诞生时间要早于项目发布时间，免杀效果截至项目发布时间已经很差了。
