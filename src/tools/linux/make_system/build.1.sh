#!/bin/bash
set -e -x


# deleta extract dir
for dir in `ls`
do
	if [ -d $dir ]; then
		rm -rf $dir
	fi
done

# show detailed information about the preprocessor, compilation, and assembly stages, including gcc's included search paths and their order.
# gcc -v test.c
cd $LFS/sources
tar -xvf binutils-2.32.tar.xz && cd binutils-2.32
mkdir -v build && cd build

# --prefix=/tools, this tells configure script to install files in the /tools directory
# --with-sysroot, this tells the build system to look in $LFS for the target system libraries as needed.
# --with-lib-path=/tools/lib, this specifies which library path the linker should be configured to use.
# --target=$LFS_TGT, in source dir, ./config.guess returns the value
# --disable-nls, this disables internationalization as i18n is not needed for the temporary tools.
# --disable-werror, this prevents the build from stopping in the event that there are warnings from the host's compiler.
../configure --prefix=/tools            \
             --with-sysroot=$LFS        \
             --with-lib-path=/tools/lib \
             --target=$LFS_TGT          \
             --disable-nls              \
             --disable-werror

make -j12

case $(uname -m) in
  x86_64) mkdir -v /tools/lib && ln -sv lib /tools/lib64 ;;
esac

make install

# build gcc
cd $LFS/sources
tar -xf gcc-8.2.0.tar.xz && cd gcc-8.2.0
tar -xf ../mpfr-4.0.2.tar.xz && mv -v mpfr-4.0.2 mpfr
tar -xf ../gmp-6.1.2.tar.xz && mv -v gmp-6.1.2 gmp
tar -xf ../mpc-1.1.0.tar.gz && mv -v mpc-1.1.0 mpc


for file in gcc/config/{linux,i386/linux{,64}}.h
do
  cp -uv $file{,.orig}
  sed -e 's@/lib\(64\)\?\(32\)\?/ld@/tools&@g' \
      -e 's@/usr@/tools@g' $file.orig > $file
  echo '
#undef STANDARD_STARTFILE_PREFIX_1
#undef STANDARD_STARTFILE_PREFIX_2
#define STANDARD_STARTFILE_PREFIX_1 "/tools/lib/"
#define STANDARD_STARTFILE_PREFIX_2 ""' >> $file
  touch $file.orig
done

case $(uname -m) in
  x86_64)
    sed -e '/m64=/s/lib64/lib/' \
        -i.orig gcc/config/i386/t-linux64
 ;;
esac

mkdir -v build && cd build


../configure                                       \
    --target=$LFS_TGT                              \
    --prefix=/tools                                \
    --with-glibc-version=2.11                      \
    --with-sysroot=$LFS                            \
    --with-newlib                                  \
    --without-headers                              \
    --with-local-prefix=/tools                     \
    --with-native-system-header-dir=/tools/include \
    --disable-nls                                  \
    --disable-shared                               \
    --disable-multilib                             \
    --disable-decimal-float                        \
    --disable-threads                              \
    --disable-libatomic                            \
    --disable-libgomp                              \
    --disable-libmpx                               \
    --disable-libquadmath                          \
    --disable-libssp                               \
    --disable-libvtv                               \
    --disable-libstdcxx                            \
    --enable-languages=c,c++

make -j12 && make install


cd $LFS/sources
tar -xf linux-4.20.12.tar.xz && cd linux-4.20.12
make mrproper
make INSTALL_HDR_PATH=dest headers_install
cp -rv dest/include/* /tools/include


cd $LFS/sources
tar -xf glibc-2.29.tar.xz && cd glibc-2.29
mkdir -v build && cd build
../configure                             \
      --prefix=/tools                    \
      --host=$LFS_TGT                    \
      --build=$(../scripts/config.guess) \
      --enable-kernel=3.2                \
      --with-headers=/tools/include
make -j12 && make install

echo 'int main(){}' > dummy.c
$LFS_TGT-gcc dummy.c
readelf -l a.out | grep ': /tools'
rm -v dummy.c a.out


cd $LFS/sources/gcc-8.2.0/build
../libstdc++-v3/configure           \
    --host=$LFS_TGT                 \
    --prefix=/tools                 \
    --disable-multilib              \
    --disable-nls                   \
    --disable-libstdcxx-threads     \
    --disable-libstdcxx-pch         \
    --with-gxx-include-dir=/tools/$LFS_TGT/include/c++/8.2.0

make -j12 && make install


cd $LFS/sources/binutils-2.32/build && rm * -rf
CC=$LFS_TGT-gcc                \
AR=$LFS_TGT-ar                 \
RANLIB=$LFS_TGT-ranlib         \
../configure                   \
    --prefix=/tools            \
    --disable-nls              \
    --disable-werror           \
    --with-lib-path=/tools/lib \
    --with-sysroot
make -j12 && make install

make -C ld clean
make -C ld LIB_PATH=/usr/lib:/lib
cp -v ld/ld-new /tools/bin


cd $LFS/sources/gcc-8.2.0
cat gcc/limitx.h gcc/glimits.h gcc/limity.h > \
  `dirname $($LFS_TGT-gcc -print-libgcc-file-name)`/include-fixed/limits.h

for file in gcc/config/{linux,i386/linux{,64}}.h
do
  cp -uv $file{,.orig}
  sed -e 's@/lib\(64\)\?\(32\)\?/ld@/tools&@g' \
      -e 's@/usr@/tools@g' $file.orig > $file
  echo '
#undef STANDARD_STARTFILE_PREFIX_1
#undef STANDARD_STARTFILE_PREFIX_2
#define STANDARD_STARTFILE_PREFIX_1 "/tools/lib/"
#define STANDARD_STARTFILE_PREFIX_2 ""' >> $file
  touch $file.orig
done

case $(uname -m) in
  x86_64)
    sed -e '/m64=/s/lib64/lib/' \
        -i.orig gcc/config/i386/t-linux64
  ;;
esac

cd build && rm * -rf

CC=$LFS_TGT-gcc                                    \
CXX=$LFS_TGT-g++                                   \
AR=$LFS_TGT-ar                                     \
RANLIB=$LFS_TGT-ranlib                             \
../configure                                       \
    --prefix=/tools                                \
    --with-local-prefix=/tools                     \
    --with-native-system-header-dir=/tools/include \
    --enable-languages=c,c++                       \
    --disable-libstdcxx-pch                        \
    --disable-multilib                             \
    --disable-bootstrap                            \
    --disable-libgomp
make -j12 && make install
ln -sv gcc /tools/bin/cc

echo 'int main(){}' > dummy.c
cc dummy.c
readelf -l a.out | grep ': /tools'


cd $LFS/sources
tar -xf tcl8.6.9-src.tar.gz && cd tcl8.6.9
cd unix
./configure --prefix=/tools
make -j12
TZ=UTC make test
make install
chmod -v u+w /tools/lib/libtcl8.6.so
make install-private-headers
ln -sv tclsh8.6 /tools/bin/tclsh


cd $LFS/sources
tar -xf expect5.45.4.tar.gz && cd expect5.45.4
cp -v configure{,.orig}
sed 's:/usr/local/bin:/bin:' configure.orig > configure
./configure --prefix=/tools       \
            --with-tcl=/tools/lib \
            --with-tclinclude=/tools/include
make -j12
# ignore failed test
make test
make SCRIPTS="" install


cd $LFS/sources
tar -xf dejagnu-1.6.2.tar.gz && cd dejagnu-1.6.2
./configure --prefix=/tools
make install
make check

cd $LFS/sources
tar -xf m4-1.4.18.tar.xz && cd m4-1.4.18
sed -i 's/IO_ftrylockfile/IO_EOF_SEEN/' lib/*.c
echo "#define _IO_IN_BACKUP 0x100" >> lib/stdio-impl.h
./configure --prefix=/tools
make -j12 && make check && make install


cd $LFS/sources
tar -xf ncurses-6.1.tar.gz && cd ncurses-6.1
sed -i s/mawk// configure
./configure --prefix=/tools \
            --with-shared   \
            --without-debug \
            --without-ada   \
            --enable-widec  \
            --enable-overwrite
make -j12 && make install
ln -s libncursesw.so /tools/lib/libncurses.so


cd $LFS/sources
tar -xf bash-5.0.tar.gz &&cd bash-5.0
./configure --prefix=/tools --without-bash-malloc
make -j12 && make tests && make install
ln -sv bash /tools/bin/sh


cd $LFS/sources
tar -xf bison-3.3.2.tar.xz && cd bison-3.3.2
./configure --prefix=/tools
make -j12 && make check && make install


cd $LFS/sources
tar -xf bzip2-1.0.6.tar.gz && cd bzip2-1.0.6
make -j12 && make PREFIX=/tools install


cd $LFS/sources
tar -xf coreutils-8.30.tar.xz && cd coreutils-8.30
./configure --prefix=/tools --enable-install-program=hostname
make -j12
# make RUN_EXPENSIVE_TESTS=yes check
make install


cd $LFS/sources
tar -xf diffutils-3.7.tar.xz && cd diffutils-3.7
./configure --prefix=/tools
make -j12
# make check
make install

cd $LFS/sources
tar -xf file-5.36.tar.gz && cd file-5.36
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf findutils-4.6.0.tar.gz && cd findutils-4.6.0
sed -i 's/IO_ftrylockfile/IO_EOF_SEEN/' gl/lib/*.c
sed -i '/unistd/a #include <sys/sysmacros.h>' gl/lib/mountlist.c
echo "#define _IO_IN_BACKUP 0x100" >> gl/lib/stdio-impl.h
./configure --prefix=/tools
make
# make check
make install


cd $LFS/sources
tar -xf gawk-4.2.1.tar.xz && cd gawk-4.2.1
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf gettext-0.19.8.1.tar.xz && cd gettext-0.19.8.1
cd gettext-tools
EMACS="no" ./configure --prefix=/tools --disable-shared
make -C gnulib-lib
make -C intl pluralx.c
make -C src msgfmt
make -C src msgmerge
make -C src xgettext
cp -v src/{msgfmt,msgmerge,xgettext} /tools/bin


cd $LFS/sources
tar -xf grep-3.3.tar.xz && cd grep-3.3
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf gzip-1.10.tar.xz && cd gzip-1.10
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf make-4.2.1.tar.bz2 && cd make-4.2.1
sed -i '211,217 d; 219,229 d; 232 d' glob/glob.c
./configure --prefix=/tools --without-guile
make -j12
# make check
make install


cd $LFS/sources
tar -xf patch-2.7.6.tar.xz && cd patch-2.7.6
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf perl-5.28.1.tar.xz && cd perl-5.28.1
sh Configure -des -Dprefix=/tools -Dlibs=-lm -Uloclibpth -Ulocincpth
make -j12
cp -v perl cpan/podlators/scripts/pod2man /tools/bin
mkdir -pv /tools/lib/perl5/5.28.1
cp -Rv lib/* /tools/lib/perl5/5.28.1


cd $LFS/sources
tar -xf Python-3.7.2.tar.xz && cd Python-3.7.2
sed -i '/def add_multiarch_paths/a \        return' setup.py
./configure --prefix=/tools --without-ensurepip
make -j12
make install


cd $LFS/sources
tar -xf sed-4.7.tar.xz && cd sed-4.7
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf tar-1.31.tar.xz && cd tar-1.31
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf texinfo-6.5.tar.xz && cd texinfo-6.5
./configure --prefix=/tools
make -j12
# make check
make install


cd $LFS/sources
tar -xf xz-5.2.4.tar.xz && cd xz-5.2.4
./configure --prefix=/tools
make -j12
# make check
make install
