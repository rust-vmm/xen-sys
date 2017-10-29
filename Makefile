
ARCH := x86_64
TARGET := $(ARCH)-xen-oxerun
CRATE := oxerun
CRATE_OBJ := $(subst -,_,$(CRATE))
CRATE_BUILD := target/$(TARGET)/debug/lib$(CRATE_OBJ).a
KERNEL := build/$(CRATE)-$(ARCH).bin

LDS := oxerun.ld
RUST_SRCS := $(wildcard **/src/**.rs)
SRCS := $(wildcard oxerun/src/$(ARCH)/*.S)
OBJS := $(patsubst oxerun/src/$(ARCH)/%.S, build/$(ARCH)/%.o, $(SRCS))

COMMON_FLAGS := -pipe -nostdinc -Ioxerun/include
AFLAGS := $(COMMON_FLAGS) -D__ASSEMBLY__

all: $(KERNEL)

clean:
	@xargo clean
	@rm -rf build

$(KERNEL): $(CRATE_BUILD) $(LDS) $(OBJS)
	ld -n --gc-sections -T $(LDS) -o $(KERNEL) $(OBJS) $(CRATE_BUILD)

$(CRATE_BUILD): $(RUST_SRCS)
	@xargo build --target $(TARGET)

build/$(ARCH)/%.o: oxerun/src/$(ARCH)/%.S
	@mkdir -p $(@D)
	$(CC) -c -o $@ $(AFLAGS) $<
