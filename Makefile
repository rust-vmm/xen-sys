
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
CRATE_OBJ := $(subst -,_,$(CRATE))
CRATE_BUILD := target/$(TARGET)/$(OPT_TARGET)/lib$(CRATE_OBJ).a
KERNEL := build/$(OPT_TARGET)/$(CRATE)-$(ARCH).bin

LDS := $(TARGET).ld
RUST_SRCS := $(wildcard **/src/**.rs)
SRCS := $(wildcard oxerun/src/$(ARCH)/*.S)
OBJS := $(patsubst oxerun/src/$(ARCH)/%.S, build/$(OPT_TARGET)/$(ARCH)/%.o, $(SRCS))

COMMON_FLAGS := -pipe -nostdinc -Ioxerun/include
AFLAGS := $(COMMON_FLAGS) -D__ASSEMBLY__

all: $(KERNEL)

clean:
	@cargo clean
	@rm -rf target/sysroot
	@rm -rf build

$(KERNEL): $(CRATE_BUILD) $(LDS) $(OBJS)
	@echo "  [LD] $(ARCH)/$(@F)"
	ld -n --gc-sections -T $(LDS) -o $(KERNEL) $(OBJS) $(CRATE_BUILD)

$(CRATE_BUILD): $(RUST_SRCS)
	@echo "  [XARGO] $(ARCH)/$(@F)"
	@cargo xbuild $(CARGO_TARGET) --target $(TARGET)

build/$(OPT_TARGET)/$(ARCH)/%.o: oxerun/src/$(ARCH)/%.S
	@mkdir -p $(@D)
	@echo "  [CC] $(ARCH)/$(@F)"
	@$(CC) -g -c -o $@ $(AFLAGS) $<
