import os
buffer = []
filename = None
def print_help():
    print("Commands:")
    print("  ld           - list directories in the current path")
    print("  cd [path]    - change directories")
    print("  a            - append text")
    print("  i[bfre]      - insert text")
    print("  p[:from:,to] - print buffer")
    print("  s [expr]     - search in buffer")
    print("  d[from:,to]  - delete from buffer")
    print("  w [file]     - write to file")
    print("  r [file]     - read from file")
    print("  q            - quit")
    print("  h            - help")
def main():
    global buffer, filename
    print("Mini-editor (type 'h' for help)")
    print("                   __        \n.--------.-----.--|  |══     \n|        |  -__|  _  |════   \n|__|__|__|_____|_____|═══════\n")

    while True:
        try:
            cmd = input("* ").strip().split()
            if not cmd:
                continue
                
            if cmd[0] == "ld":
                print(f"\tCurrent dir: {os.getcwd()}")
                for i in os.listdir():
                    print(f"./{i}")
            elif cmd[0] == "cd":
                if len(cmd) > 1:
                    path = cmd[1]
                if not path:
                    print("Error: no path specified")
                    continue
                try:
                    os.chdir(path)
                except FileNotFoundError:
                    print("Directory does not exist!")
                except PermissionError:
                    print("Permission denied to access the directory!")
            elif cmd[0] == 'a':
                print("Enter text (single '.' on a line to finish):")
                while True:
                    line = input()
                    if line == '.':
                        break
                    buffer.append(line)
            elif cmd[0][0] == 'i':
                if len(cmd[0]) <= 1:
                    print("Error: you want to insert text before what?")
                    continue
                try:
                    if (int(cmd[0][1:]) < 1) or (int(cmd[0][1:]) > len(buffer)):
                        print("Error: invalid line index")
                        continue
                except ValueError:
                    print("Error: invalid line index")
                    continue
                print("Enter text (single '.' on a line to finish):")
                index = int(cmd[0][1:])
                while True:
                    line = input()
                    if line == '.':
                        break
                    buffer.insert(index-1, line)
                    index += 1
            elif cmd[0][0] == 'p':
                try:
                    args = cmd[0][1:].split(",")
                    if args[0] == '':
                        for i, line in enumerate(buffer, 1):
                            print(f"{str(i).rjust(len(str(len(buffer))))}: {line.replace("\t", "\033[90m|   \033[0m").replace("    ", "\033[90m|   \033[0m")}")
                    elif len(args) == 1:
                        args = int(args[0])
                        print(f"{str(args).rjust(len(str(len(buffer))))}: {buffer[args-1].replace("\t", "\033[90m|   \033[0m").replace("    ", "\033[90m|   \033[0m")}")
                    else:
                        args = [int(args[0]), int(args[1])]
                        if args[0] <= args[1]:
                            for i, line in enumerate(buffer[args[0]-1:], args[0]):
                                if i > args[1]:
                                    break
                                print(f"{str(i).rjust(len(str(len(buffer))))}: {line.replace("\t", "\033[90m|   \033[0m").replace("    ", "\033[90m|   \033[0m")}")
                        else: print("Error: non-existent interval")
                except ValueError:
                    print("Error: invalid line index")
                except:
                    print("Syntax error")
            elif cmd[0] == 's':
                if len(cmd) > 1:
                    for i, buf in enumerate(buffer, 1):
                        right = 1
                        for word in buf.lower().split():
                            if word == cmd[right].lower():
                                right += 1
                            if right >= len(cmd):
                                print(f"{str(i).rjust(len(str(len(buffer))))}: {buf.replace("\t", "\033[90m|   \033[0m").replace("    ", "\033[90m|   \033[0m")}")
                                break
            elif cmd[0][0] == 'd':
                try:
                    args = cmd[0][1:].split(",")
                    if len(args) == 1:
                        args = int(args[0])
                        buffer.pop(args-1)
                    else:
                        args = [int(args[0]), int(args[1])]
                        if args[0] <= args[1]:
                            for i in range(args[0]-1, args[1]):
                                buffer.pop(args[0]-1)
                        else: print("Error: non-existent interval")
                except ValueError:
                    print("Error: invalid line index")
                    continue
                except:
                    print("Syntax error")
            elif cmd[0] == 'w':
                if len(cmd) > 1:
                    filename = cmd[1]
                if not filename:
                    print("Error: no filename specified")
                    continue
                with open(filename, 'w') as f:
                    f.write('\n'.join(buffer))
                print(f"Wrote {len(buffer)} lines to {filename}")
                
            elif cmd[0] == 'r':
                if len(cmd) > 1:
                    filename = cmd[1]
                if not filename:
                    print("Error: no filename specified")
                    continue
                try:
                    with open(filename, 'r') as f:
                        buffer = f.read().splitlines()
                    print(f"Read {len(buffer)} lines from {filename}")
                except FileNotFoundError:
                    print(f"Error: file {filename} not found")
                    
            elif cmd[0] == 'q':
                break
                
            elif cmd[0] == 'h':
                print_help()
                
            else:
                print("Unknown command. Type 'h' for help.")
                
        except EOFError:
            print()
            break
        except KeyboardInterrupt:
            print("\nInterrupted")
            break
main()
