import labels
from reportlab.graphics import shapes

import os


def draw_label(label, width, height, obj):
    for group in obj:
        for index, item in enumerate(group[::-1]):
            label.add(
                shapes.String(
                    width // 2,
                    11 * index + 9,
                    item,
                    fontName="Helvetica",
                    fontSize=10,
                    textAnchor="middle",
                )
            )


# function that returns the values of all entries
def print_label(data):
    path = "pdf"
    # Check whether the specified path exists or not
    isExist = os.path.exists(path)
    if not isExist:
        # Create a new directory because it does not exist
        os.makedirs(path)

    # LABEL FUNCTIONS
    specification = labels.Specification(
        216,
        279,
        3,
        10,
        66.75,
        25.4,
        top_margin=12.46,
        row_gap=0,
        column_gap=3,
        corner_radius=2,
    )
    sheet = labels.Sheet(specification, draw_label, border=True)

    output = []

    # Quantity required
    qty_req = data.get("specifyQty")
    qty = data.get("specifiedQty")

    # Get the content of the label
    content = data.get("rows")

    # Get the serial numbers
    sn_entry_window = data.get("serialNumberList")

    if len(sn_entry_window) > 0:
        # format values for printing w/ serial numbers
        for index, serial_number in enumerate(sn_entry_window):
            row = []
            for item in content:
                for k, v in item.items():
                    row.append(f"{k.upper()}:{v.upper()}")
            output.append([row])
            output[index][0].append(f"SN:{serial_number.upper()}")
    else:
        # format values for printing w/out serial numbers
        row = []
        for item in content:
            for k, v in item.items():
                row.append(f"{k.upper()}:{v.upper()}")
        output.append([row])

    # join items that share a line
    output_formatted = [[]]
    for group in output:
        subgroup = {i.split(":")[0]: i.split(":")[1] for i in group[0]}
        pop_list = [f"{i}:{j}" if j else None for i, j in subgroup.items()]
        pop_list = list(filter(None, pop_list))
        qty_text = f"  QTY:{qty}" if qty_req else ""

        ### add items to list, account for missing items

        # pop_list contains only populated fields in dictionary format
        def try_value(value):
            try:
                if pop_list[value].split(":")[0] in ["CLASS", "PN", "SN"]:
                    return pop_list[value].split(":")[1]
                return pop_list[value]
            except IndexError:
                return ""

        output_formatted.append(
            [
                [
                    f"{try_value(0)} " f" {try_value(1)}",
                    f"{try_value(2)} " f" {try_value(3)}",
                    f"{try_value(4)} " f" {try_value(5)}",
                    f"{try_value(6)}",
                    f"{try_value(7).upper()} " f"{qty_text}",
                ]
            ]
        )

    # make a label for each item in output
    for group in output_formatted:
        group = [*group]
        print(group)
        if group:
            sheet.add_label(group)
    sheet.save(f"{path}/labels.pdf")

    return "Label printed successfully!"
