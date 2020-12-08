AR ?= ar
AS ?= as
CC ?= gcc
CXX ?= g++

CXXFLAGS = $(CFLAGS) -std=c++2a -O2 -fPIC -Wall -Wextra -Werror

SUBDIRS = src
