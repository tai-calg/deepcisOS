#OUTPUT_DIR := build
#ASM_DIR := asm
#OUTPUT_DIR_KEEP := $(OUTPUT_DIR)/.keep
#
#$(OUTPUT_DIR)/ipl.bin: $(ASM_DIR)/ipl.asm Makefile $(OUTPUT_DIR_KEEP)
#$(OUTPUT_DIR)/asmhead.bin: $(ASM_DIR)/asmhead.asm Makefile $(OUTPUT_DIR_KEEP)
#
#$(OUTPUT_DIR)/%.bin: $(ASM_DIR)/%.asm Makefile $(OUTPUT_DIR_KEEP)
#	nasm $< -o $@
#
#$(OUTPUT_DIR_KEEP):
#	mkdir -p $(OUTPUT_DIR)
#	touch $@