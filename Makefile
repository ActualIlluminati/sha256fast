AR ?= ar
AS ?= as
CC ?= gcc
CXX ?= g++

all: run-generic run-ssse3 run-avx run-avx2

clean: run-generic run-ssse3 run-avx run-avx2
	-rm -vf $^

%.o: %.S
	$(CXX) -o $@ $< -c $(CXXFLAGS) -Wa,-mrelax-relocations=yes

%.o: %.cxx
	$(CXX) -std=c++2a -o $@ $< -c $(CXXFLAGS)

run-generic: sha256-generic.o main.o
	$(CXX) -o $@ $^ $(CXXFLAGS) -no-pie

run-ssse3: sha256-ssse3-asm.o sha256-ssse3.o main.o
	$(CXX) -o $@ $^ $(CXXFLAGS) -no-pie

run-avx: sha256-avx-asm.o sha256-avx.o main.o
	$(CXX) -o $@ $^ $(CXXFLAGS) -no-pie

run-avx2: sha256-avx2-asm.o sha256-avx2.o main.o
	$(CXX) -o $@ $^ $(CXXFLAGS) -no-pie

libsha256fast.a: sha256-avx2-asm.o sha256-avx2.o
	$(AR) rc $@ $^
