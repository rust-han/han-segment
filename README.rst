中文分词系统
==============

:Date: 10/07 2018

.. contents::


介绍
------

一个使用 `Rust` 语言实现的中文分词系统。


算法
------------

1. 隐私马尔可夫模型（HMM）
2. 基于字典的正向最大化匹配（MMSEG）


字典来源
------------

1. MMSEG 中文分词字典来源于 `chenlb/mmseg4j-from-googlecode <https://github.com/chenlb/mmseg4j-from-googlecode>`_ 。
2. HMM 中文分词算法所使用到的模型数据来源于 `yanyiwu/cppjieba <https://github.com/yanyiwu/cppjieba>`_ 。


其它相关项目
------------

*   `fxsjy/jieba <https://github.com/fxsjy/jieba>`_ , 结巴中文分词
*   `chenlb/mmseg4j-from-googlecode <https://github.com/chenlb/mmseg4j-from-googlecode>`_ , MMSEG 中文分词 (Java)
*   `archerhu/scel2mmseg <https://github.com/archerhu/scel2mmseg>`_ , 一个搜狗细胞词库转换为MMSEG词库的工具
*   `baidu/lac <https://github.com/baidu/lac>`_ , 中文词法分析（LAC）
*   `baidu/AnyQ <https://github.com/baidu/AnyQ>`_ , 百度FAQ自动问答系统
*   `baidu/Senta <https://github.com/baidu/Senta>`_ , 百度情感识别系统


参考
------

*   `MMSEG <http://technology.chtsai.org/mmseg/>`_ , A Word Identification System for Mandarin Chinese Text Based on Two Variants of the Maximum Matching Algorithm
*   `国家语委现代汉语语料库 <http://www.cncorpus.org/index.aspx>`_
*   `互联网上开放的中文语料库有哪些 <https://www.zhihu.com/question/21177095>`_
*   `搜狗实验室_语料数据 <https://www.sogou.com/labs/resource/list_yuliao.php>`_