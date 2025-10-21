# Makefile for building optical flow executables
# Requires OpenCV 4.x with C++ support

CXX = g++
CXXFLAGS = -std=c++11 -O2
OPENCV_FLAGS = $(shell pkg-config --cflags --libs opencv4 2>/dev/null || pkg-config --cflags --libs opencv)

# Check if OpenCV is available
ifeq ($(OPENCV_FLAGS),)
    $(error OpenCV not found. Please install OpenCV 4.x and ensure pkg-config can find it)
endif

# Executables to build
EXECUTABLES = lucas_kanade farneback

# Default target
all: $(EXECUTABLES)

# Build lucas_kanade executable
lucas_kanade: lucas_kanade.cpp
	$(CXX) $(CXXFLAGS) -o lucas_kanade lucas_kanade.cpp $(OPENCV_FLAGS)

# Build farneback executable
farneback: farneback.cpp
	$(CXX) $(CXXFLAGS) -o farneback farneback.cpp $(OPENCV_FLAGS)

# Clean up built executables
clean:
	rm -f $(EXECUTABLES)

# Test the executables (requires test images)
test: $(EXECUTABLES)
	@echo "Testing executables..."
	@if [ -f "../../input/1.png" ] && [ -f "../../input/2.png" ]; then \
		echo "Testing lucas_kanade..."; \
		./lucas_kanade ../../input/1.png ../../input/2.png > /dev/null && echo "✓ lucas_kanade works"; \
		echo "Testing farneback..."; \
		./farneback ../../input/1.png ../../input/2.png > /dev/null && echo "✓ farneback works"; \
	else \
		echo "⚠️  Test images not found. Run 'make extract-frames' first or extract frames manually."; \
	fi

# Extract test frames (calls the frame extraction script)
extract-frames:
	@echo "Extracting test frames..."
	@cd ../../.. && chmod +x scripts/extract_frames.sh && ./scripts/extract_frames.sh

# Install target (copy executables to system PATH - optional)
install: $(EXECUTABLES)
	@echo "Installing executables to /usr/local/bin (requires sudo)"
	sudo cp $(EXECUTABLES) /usr/local/bin/

# Help target
help:
	@echo "Available targets:"
	@echo "  all          - Build all executables (default)"
	@echo "  lucas_kanade - Build lucas_kanade executable"
	@echo "  farneback    - Build farneback executable"
	@echo "  test         - Test the executables with sample images"
	@echo "  extract-frames - Extract test frames from video"
	@echo "  clean        - Remove built executables"
	@echo "  install      - Install executables to system PATH"
	@echo "  help         - Show this help message"

.PHONY: all clean test extract-frames install help
