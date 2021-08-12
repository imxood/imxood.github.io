# qwt笔记

[Qwt User's Guide](https://qwt.sourceforge.io/qwtinstall.html)


## 安装 qwt

    svn checkout https://svn.code.sf.net/p/qwt/code/trunk qwt-code

    cd qwt-code/qwt

    qmake

    make -j10

    sudo make install

## 添加 qwt到qt designer中

    cp /usr/local/qwt-6.3.0-svn/plugins/designer/libqwt_designer_plugin.so Qt5.14.0/Tools/QtCreator/lib/Qt/plugins/designer