import cv2

cv2.waitKey(100)
for i in range(16):
    path = "cmake-build-debug/" + str(i) + "-test.ppm"
    img = cv2.imread(path)
    cv2.imshow("anim", img)
    cv2.waitKey(40)

cv2.destroyAllWindows()