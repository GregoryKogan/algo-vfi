#include <opencv2/opencv.hpp>
#include <iostream>
#include <vector>

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

    // Detect corners in the first image using goodFeaturesToTrack
    std::vector<cv::Point2f> corners;
    cv::goodFeaturesToTrack(img1, corners, 1000, 0.01, 10);

    if (corners.empty()) {
        // If no corners found, create a grid of points
        for (int y = 0; y < height; y += 8) {
            for (int x = 0; x < width; x += 8) {
                corners.push_back(cv::Point2f(x, y));
            }
        }
    }

    // Calculate optical flow using Lucas-Kanade
    std::vector<cv::Point2f> next_corners;
    std::vector<uchar> status;
    std::vector<float> error;
    
    cv::calcOpticalFlowPyrLK(img1, img2, corners, next_corners, status, error);

    // Create a dense flow field by interpolating from sparse points
    cv::Mat flow_field(height, width, CV_32FC2);
    flow_field.setTo(cv::Scalar(0, 0));

    // Fill the flow field with interpolated values
    for (size_t i = 0; i < corners.size(); i++) {
        if (status[i] && error[i] < 50) { // Only use good matches
            cv::Point2f flow = next_corners[i] - corners[i];
            
            // Apply flow to a small region around the point
            int x = static_cast<int>(corners[i].x);
            int y = static_cast<int>(corners[i].y);
            
            for (int dy = -4; dy <= 4; dy++) {
                for (int dx = -4; dx <= 4; dx++) {
                    int nx = x + dx;
                    int ny = y + dy;
                    if (nx >= 0 && nx < width && ny >= 0 && ny < height) {
                        flow_field.at<cv::Vec2f>(ny, nx) = cv::Vec2f(flow.x, flow.y);
                    }
                }
            }
        }
    }

    // Output the flow field in the expected format
    std::cout << height << " " << width << std::endl;
    
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            cv::Vec2f flow = flow_field.at<cv::Vec2f>(y, x);
            std::cout << flow[0] << " " << flow[1] << std::endl;
        }
    }

    return 0;
}
