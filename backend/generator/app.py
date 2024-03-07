from reportlab.graphics import shapes
import json
import labels
import os

OUTPUT_DIRECTORY = "pdf"
OUTPUT_FILENAME = "labels.pdf"

FONT_SIZE = 10
LINES_PER_LABEL = 5

SHEET_WIDTH = 216
SHEET_HEIGHT = 279
COLUMNS = 3
ROWS = 10
LABEL_WIDTH = 66.75
LABEL_HEIGHT = 25.4
TOP_MARGIN=12.46
ROW_GAP=0
COLUMN_GAP=3
CORNER_RADIUS=2

def build_specification(**kwargs):
    """Build a label specification."""
    return labels.Specification(
        kwargs.get("sheet_width", SHEET_WIDTH),
        kwargs.get("sheet_height", SHEET_HEIGHT),
        kwargs.get("columns", COLUMNS),
        kwargs.get("rows", ROWS),
        kwargs.get("label_width", LABEL_WIDTH),
        kwargs.get("label_height", LABEL_HEIGHT),
        top_margin=kwargs.get("top_margin", TOP_MARGIN),
        row_gap=kwargs.get("row_gap", ROW_GAP),
        column_gap=kwargs.get("column_gap", COLUMN_GAP),
        corner_radius=kwargs.get("corner_radius", CORNER_RADIUS),
    )

def draw_label(label, width, _height, obj):
    """Draw a single label."""
    y_offset = len(obj) - LINES_PER_LABEL
    for index, item in enumerate(obj[::-1], start=1):
        label.add(
            shapes.String(
                width // 2,
                (11 * index) - y_offset * FONT_SIZE//2,
                item,
                fontName="Helvetica",
                fontSize=FONT_SIZE,
                textAnchor="middle",
            )
        )

def join_items(line):
    return " ".join([f"{k}:{v}" for k, v in line.items()])

def print_label(data):
    if not os.path.exists(OUTPUT_DIRECTORY):
        os.makedirs(OUTPUT_DIRECTORY)

    # TODO: Get sheet specification from data
    sheet = labels.Sheet(build_specification(), draw_label, border=True)
    output = []

    print_multiple = data.get("printMultiple")
    amount_per_label = data.get("amountPerLabel")
    specify_qty = data.get("specifyQty")
    specified_qty = data.get("specifiedQty")
    
    rows = data.get("rows")
    serial_numbers = data.get("serialNumberList")

    # Format data when serial numbers are present
    if serial_numbers:
        for index in range(len(serial_numbers)):
            current_sn = serial_numbers[index]
            output.append([*rows, {"SN":current_sn}])
    else:
        output.append([*rows])

    output_formatted = []
    for label_details in output:

        contents = []
        for index, line in enumerate(label_details, start=1):
            line_content = join_items(line)
            if index == len(label_details) and specify_qty:
                line_content += f" QTY:{specified_qty}"

            contents.append(line_content)

        output_formatted.append(contents)
    
    for label in output_formatted:
        sheet.add_label(label, amount_per_label if print_multiple else 1)

    output_path = os.path.join(OUTPUT_DIRECTORY, OUTPUT_FILENAME)
    sheet.save(output_path)

    return f"Output generated at: {output_path}"

def run(*args):
    """Run the generator."""
    data_dict = json.loads(args[0])
    return print_label(data_dict)
