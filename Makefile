NAME := yogiwm

SRC_DIR   := src
LIB_DIR   := lib
BUILD_DIR := build
BIN_DIR   := bin

PROTO_GEN_DIR := protocols/generated

CC := gcc
PKG_CONFIG := pkg-config
WAYLAND_SCANNER := wayland-scanner

PKGS := wlroots-0.19 wayland-server xkbcommon libinput wayland-protocols xcb xcb-icccm
DEBUG ?= 0

WAYLAND_PROTOCOLS := $(shell $(PKG_CONFIG) --variable=pkgdatadir wayland-protocols)

# -----------------------------------------------------------------------------
# Generated protocol headers
# -----------------------------------------------------------------------------
PROTO_HDRS := \
	$(PROTO_GEN_DIR)/xdg-shell-protocol.h \
	$(PROTO_GEN_DIR)/wlr-layer-shell-unstable-v1-protocol.h \
	$(PROTO_GEN_DIR)/cursor-shape-v1-protocol.h \
	$(PROTO_GEN_DIR)/wlr-output-power-management-unstable-v1-protocol.h \
	$(PROTO_GEN_DIR)/pointer-constraints-unstable-v1-protocol.h

# -----------------------------------------------------------------------------
# Sources
# -----------------------------------------------------------------------------
SRCS := $(sort \
	$(wildcard $(SRC_DIR)/*.c) \
	$(wildcard $(LIB_DIR)/*.c) \
	$(wildcard $(LIB_DIR)/*/*.c))

OBJS := $(addprefix $(BUILD_DIR)/,$(SRCS:.c=.o))
DEPS := $(OBJS:.o=.d)

PKG_CFLAGS := $(shell $(PKG_CONFIG) --cflags $(PKGS))
PKG_LIBS   := $(shell $(PKG_CONFIG) --libs   $(PKGS))

# -----------------------------------------------------------------------------
# Flags
# -----------------------------------------------------------------------------
CPPFLAGS := \
	-I. -Iinclude -I$(SRC_DIR) -I$(LIB_DIR) -I$(PROTO_GEN_DIR) \
	-DWLR_USE_UNSTABLE -DXWAYLAND

CFLAGS := $(PKG_CFLAGS) -Wall -Wextra -Wpedantic

ifeq ($(DEBUG),1)
CFLAGS += -O0 -g -Wshadow -Werror=implicit -Werror=return-type
else
CFLAGS += -Os
endif

LDLIBS := $(PKG_LIBS) -lm

# -----------------------------------------------------------------------------
# Targets
# -----------------------------------------------------------------------------
all: $(BIN_DIR)/$(NAME)

debug:
	$(MAKE) DEBUG=1

$(BIN_DIR)/$(NAME): $(PROTO_HDRS) $(OBJS) | $(BIN_DIR)
	$(CC) $(OBJS) -o $@ $(LDLIBS)

# -----------------------------------------------------------------------------
# Protocol generation
# -----------------------------------------------------------------------------

$(PROTO_GEN_DIR)/xdg-shell-protocol.h:
	mkdir -p $(PROTO_GEN_DIR)
	$(WAYLAND_SCANNER) server-header \
	$(WAYLAND_PROTOCOLS)/stable/xdg-shell/xdg-shell.xml $@

$(PROTO_GEN_DIR)/cursor-shape-v1-protocol.h:
	mkdir -p $(PROTO_GEN_DIR)
	$(WAYLAND_SCANNER) server-header \
	$(WAYLAND_PROTOCOLS)/staging/cursor-shape/cursor-shape-v1.xml $@

$(PROTO_GEN_DIR)/wlr-layer-shell-unstable-v1-protocol.h:
	mkdir -p $(PROTO_GEN_DIR)
	$(WAYLAND_SCANNER) server-header \
	protocols/xml/wlr-layer-shell-unstable-v1.xml $@


$(PROTO_GEN_DIR)/wlr-output-power-management-unstable-v1-protocol.h:
	mkdir -p $(PROTO_GEN_DIR)
	$(WAYLAND_SCANNER) server-header \
	protocols/xml/wlr-output-power-management-unstable-v1.xml $@

$(PROTO_GEN_DIR)/pointer-constraints-unstable-v1-protocol.h:
	mkdir -p $(PROTO_GEN_DIR)
	$(WAYLAND_SCANNER) server-header \
	$(WAYLAND_PROTOCOLS)/unstable/pointer-constraints/pointer-constraints-unstable-v1.xml $@



# -----------------------------------------------------------------------------
# Build rules
# -----------------------------------------------------------------------------
$(BUILD_DIR)/%.o: %.c
	mkdir -p $(@D)
	$(CC) $(CPPFLAGS) $(CFLAGS) -MMD -MP -c $< -o $@

$(BIN_DIR):
	mkdir -p $@

clean:
	rm -rf $(BUILD_DIR) $(BIN_DIR) $(PROTO_GEN_DIR)

-include $(DEPS)

.PHONY: all clean debug