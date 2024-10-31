This program reads live input from your microphone, and transmits each spoken word from the microphone to the serial monitor as soon as possible. 

The installation guide for this program is down below:
1) Install a linux distribution, preferably debian.
2) Buy a microphone and successfully connect to the computer.
3) Download this project from github.
4) Open the terminal in this project
5) Open the web browser and choose a speech recognition language model from there: https://alphacephei.com/vosk/models
6) I chose vosk-model-small-en-us-0.15 and click on it to download.
7) Move the downloaded model to the project directory.
8) Unpack it, rename the unpacked directory to small-stt-model
9) install the libudev package: $ sudo apt-get install libudev-dev
10) Install the vosk package (voice engine) (gotta fully describe this step later)
11) run the program with: $ cargo run small-stt-model grandma.wav 
