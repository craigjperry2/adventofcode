def main():
    rotations = """
    L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"""
    print("Hello, World!")


fn lines(text: String) -> List[StringSlice[origin_of(text)]]:
    """Split text into lines."""
    return text.splitlines()


fn paragraphs(text: String) -> List[StringSlice[origin_of(text)]]:
    """Split text into paragraphs."""
    return text.split("\n\n")


fn whole(text: String) -> List[String]:
    """The whole text."""
    return List(text)


fn identity(s: String) -> String:
    """Identity function that returns the string unchanged."""
    return s


from utils import Variant


fn parse(day_or_text: Variant[Int, String], show: Int = 8) raises -> List[String]:
    """Split the input text into sections, and apply parser to each.
    The first argument is either the text itself, or the day number of a text file.
    """
    var show_count = show

    # By default, don't show lines when parsing example text
    if day_or_text.isa[String]() and show == 8:
        show_count = 0

    var text = get_text(day_or_text)
    var text_lines = text.splitlines()
    show_items("Puzzle input", text_lines, show_count)

    var sections = lines(text.rstrip())
    var records = List[String]()
    for section in sections:
        records.append(section)

    # Only show parsed representation if using non-default parser/sections
    # (This check is harder in Mojo without runtime function comparison)
    if show_count > 0:
        show_items("Parsed representation", records, show_count)

    return records^


alias current_year: Int = 2023  # Subdirectory name for input files


fn get_text(day_or_text: Variant[Int, String]) raises -> String:
    """The text used as input to the puzzle: either a string or the day number,
    which denotes the file 'AOC/year/input{day}.txt'."""
    if day_or_text.isa[String]():
        return day_or_text[String]
    else:
        var day = day_or_text[Int]
        var filename = "AOC/" + str(current_year) + "/input" + str(day) + ".txt"
        with open(filename, "r") as f:
            return f.read()


fn show_items(source: String, items: List[String], show: Int):
    """Show the first few items, in a pretty format."""
    if show > 0:
        var hr = String("─") * 100
        var count = len(items)
        print(hr)
        print(source + " ➜ " + str(count) + " items:")
        print(hr)
        var limit = min(show, count)
        for i in range(limit):
            print(truncate(items[i]))
        if show < count:
            print("...")


fn truncate(s: String, width: Int = 100) -> String:
    """Truncate string to specified width."""
    if len(s) <= width:
        return s
    return s[:width - 3] + "..."


fn ints(text: String) raises -> List[Int]:
    """A list of all the integers in text, ignoring non-number characters."""
    var result = List[Int]()
    var current = String("")
    var in_number = False
    
    for i in range(len(text)):
        var c = text[i]
        var is_digit = c >= "0" and c <= "9"
        var is_minus = c == "-"
        
        if is_digit or (is_minus and not in_number):
            current += c
            in_number = True
        elif in_number:
            if len(current) > 0 and current != "-":
                result.append(atol(current))
            current = String("")
            in_number = False
    
    # Don't forget the last number
    if len(current) > 0 and current != "-":
        result.append(atol(current))
    
    return result^


fn positive_ints(text: String) raises -> List[Int]:
    """A list of all the positive integers in text, ignoring non-number characters."""
    var result = List[Int]()
    var current = String("")
    
    for i in range(len(text)):
        var c = text[i]
        var is_digit = c >= "0" and c <= "9"
        
        if is_digit:
            current += c
        elif len(current) > 0:
            result.append(atol(current))
            current = String("")
    
    if len(current) > 0:
        result.append(atol(current))
    
    return result^


fn digits(text: String) raises -> List[Int]:
    """A list of all the digits in text (as ints 0–9), ignoring non-digit characters."""
    var result = List[Int]()
    
    for i in range(len(text)):
        var c = text[i]
        if c >= "0" and c <= "9":
            result.append(atol(c))
    
    return result^


fn words(text: String) -> List[String]:
    """A list of all the alphabetic words in text, ignoring non-letters."""
    var result = List[String]()
    var current = String("")
    
    for i in range(len(text)):
        var c = text[i]
        var is_alpha = (c >= "a" and c <= "z") or (c >= "A" and c <= "Z")
        
        if is_alpha:
            current += c
        elif len(current) > 0:
            result.append(current)
            current = String("")
    
    if len(current) > 0:
        result.append(current)
    
    return result^


fn atom(text: String) raises -> Variant[String, Float64, Int]:
    """Parse text into a single float or int or str."""
    var trimmed = text.strip()
    
    # Try to parse as integer first
    var is_int = True
    var has_chars = False
    for i in range(len(trimmed)):
        var c = trimmed[i]
        has_chars = True
        if not ((c >= "0" and c <= "9") or (i == 0 and c == "-")):
            is_int = False
            break
    
    if is_int and has_chars:
        return atol(trimmed)
    
    # Try to parse as float
    var is_float = True
    var dot_count = 0
    for i in range(len(trimmed)):
        var c = trimmed[i]
        if c == ".":
            dot_count += 1
        elif not ((c >= "0" and c <= "9") or (i == 0 and c == "-")):
            is_float = False
            break
    
    if is_float and dot_count <= 1 and has_chars:
        return atof(trimmed)
    
    return trimmed
