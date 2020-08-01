
# override for release builds
DEBUG ?= y

CARGO_TARGET :=
OPT_TARGET := debug

ifneq ($(DEBUG),y)
OPT_TARGET := release
CARGO_TARGET := --release
endif

ARCH := x86_64
TARGET := $(ARCH)-xen-pv
CRATE := oxerun
KERNEL := build/$(OPT_TARGET)/$(CRATE)-$(ARCH).bin

RUST_SRCS := $(wildcard **/src/**.rs)

XEN_PATH ?= ../xen
XEN_INCLUDE = $(XEN_PATH)/xen/include

all: $(KERNEL)

clean:
	@cargo clean
	@rm -rf target/sysroot
	@rm -rf build

$(KERNEL): $(RUST_SRCS)
	@echo "  [CARGO] $(TARGET)/$(@F)"
	@cargo build -Z build-std=core,alloc $(CARGO_TARGET) --target ./$(TARGET).json

xen-sys/src/$(ARCH)/bindgen.rs: $(XEN_INCLUDE)/public/xen.h xen-sys/wrapper.h
	@mkdir -p $(@D)
	@bindgen \
		--rust-target nightly \
		--use-core \
		--ctypes-prefix cty \
		-o $@ \
		xen-sys/wrapper.h \
		-- -I$(XEN_INCLUDE)
