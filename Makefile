# Project settings
TARGET := yogiwm
SRC_DIR := src
INC_DIR := include
BUILD_DIR := build

# Tools
CC := gcc
PKG := pkg-config
PKGS := wlroots-0.20 wayland-server xkbcommon

# Flags from pkg-config
CFLAGS := -g -I$(INC_DIR) $(shell $(PKG) --cflags $(PKGS)) -DWLR_USE_UNSTABLE
# CFLAGS := -Wall -Wextra -g -I$(INC_DIR) $(shell $(PKG) --cflags $(PKGS)) -DWLR_USE_UNSTABLE
LDFLAGS := $(shell $(PKG) --libs $(PKGS))

# Sources and objects
SRCS := $(wildcard $(SRC_DIR)/*.c)
OBJS := $(patsubst $(SRC_DIR)/%.c, $(BUILD_DIR)/%.o, $(SRCS))

# Default rule
all: $(TARGET)

# Link executable
$(TARGET): $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

# Compile source to object
$(BUILD_DIR)/%.o: $(SRC_DIR)/%.c | $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Ensure build dir exists
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Clean up
clean:
	rm -rf $(BUILD_DIR) $(TARGET)

.PHONY: all clean