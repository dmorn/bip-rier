[![CI/CD](https://github.com/dmorn/bip-rier/actions/workflows/ci.yaml/badge.svg)](https://github.com/dmorn/bip-rier/actions/workflows/ci.yaml)

- https://www.usb.org/hid
- https://github.com/libusb/hidapi/releases
- https://cmake.org/cmake/help/latest/guide/tutorial/A%20Basic%20Starting%20Point.html


- Check page 7-5 of the manual. The scanner cannot be in the default Keyboard
  mode otherwise the OS will prevent libhid from opening it (it will take
  ownership of the device). In scanner's manual, section USB Host Parameters,
  USB Device Type: scan the "IBM Table-Top USB" code to fix it.
- https://github.com/pteich/usbsymbolreader
- The program that needs to be opened is edrawings