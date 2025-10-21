#include <opencv2/opencv.hpp>
#include <iostream>

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: " << argv[0] << " <image1> <image2>" << std::endl;
        return 1;
    }

    // Load images
    cv::Mat img1 = cv::imread(argv[1], cv::IMREAD_GRAYSCALE);
    cv::Mat img2 = cv::imread(argv[2], cv::IMREAD_GRAYSCALE);

    if (img1.empty() || img2.empty()) {
        std::cerr << "Error: Could not load images" << std::endl;
        return 1;
    }

    // Ensure both images have the same size
    if (img1.size() != img2.size()) {
        std::cerr << "Error: Images must have the same dimensions" << std::endl;
        return 1;
    }

    int height = img1.rows;
    int width = img1.cols;

    // Calculate optical flow using Gunnar Farneback's algorithm
    cv::Mat flow;
    cv::calcOpticalFlowFarneback(
        img1, img2, flow,
        0.5,  // pyramid scale
        3,    // number of pyramid levels
        15,   // window size
        3,    // number of iterations
        5,    // polynomial expansion
        1.2,  // Gaussian standard deviation
        0     // flags
    );

    // Output the flow field in the expected format
    std::cout << height << " " << width << std::endl;
    
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            cv::Point2f flow_vec = flow.at<cv::Point2f>(y, x);
            std::cout << flow_vec.x << " " << flow_vec.y << std::endl;
        }
    }

    return 0;
}
