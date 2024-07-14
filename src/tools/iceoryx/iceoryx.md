git clone https://github.com/eclipse-iceoryx/iceoryx.git

cd iceoryx

cmake -Bbuild -Hiceoryx_meta -DBUILD_TEST=ON -DINTROSPECTION=OFF -DBINDING_C=ON -DEXAMPLES=ON

cmake --build build -j 20
