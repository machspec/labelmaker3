from reportlab.graphics import shapes
import json
import labels
import os

OUTPUT_PATH = "pdf"

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

def run(*args):
    """Run the generator."""
    data_dict = json.loads(args[0])
    return print_label(data_dict)


def draw_label(label, width, _height, obj):
    """Draw a single label."""
    for index, item in enumerate(obj[::-1], start=1):
        label.add(
            shapes.String(
                width // 2,
                11 * index,
                item,
                fontName="Helvetica",
                fontSize=10,
                textAnchor="middle",
            )
        )

def join_items(line):
    return " ".join([f"{k}:{v}" for k, v in line.items()])

def print_label(data):
    if not os.path.exists(OUTPUT_PATH):
        os.makedirs(OUTPUT_PATH)

    # TODO: Get sheet specification from data
    sheet = labels.Sheet(build_specification(), draw_label, border=True)
    output = []

    print_multiple = data.get("printMultiple")
    amount_per_label = data.get("amountPerLabel")

    specify_qty = data.get("specifyQty")
    sqecified_qty = data.get("specifiedQty")
    
    rows = data.get("rows")
    serial_numbers = data.get("serialNumberList")

    # Format data when serial numbers are present
    if serial_numbers:
        for _ in range(len(serial_numbers)):
            current_sn = serial_numbers.pop(0)
            output.append([*rows, {"SN":current_sn}])
    else:
        output.append([*rows])

    output_formatted = []
    for label_details in output:

        contents = []
        for index, line in enumerate(label_details, start=1):
            line_content = join_items(line)
            if index == len(label_details) and specify_qty:
                line_content += f" QTY:{sqecified_qty}"

            contents.append(line_content)

        output_formatted.append(contents)
    
    for label in output_formatted:
        sheet.add_label(label, amount_per_label if print_multiple else 1)

    output_path = f"{OUTPUT_PATH}/labels.pdf"
    sheet.save(output_path)

    return f"Output generated at: {output_path}"
