import os
import multiprocessing
import threading
import tracemalloc

def ram():
    vec = []
    while True:
        print("DIE")
        vec.append(bytearray(1024*1024))
        pass


def infinite_threads():
    while True:
        ram()
        pass
def multi_core():
    while True:
        thread = threading.Thread(target=infinite_threads)
        thread.start()
        thread.join()
        pass



if __name__ =="__main__":
    for i in range (os.cpu_count()):
        multiprocessing.Process(target=multi_core).start()
    while True:
        pass



