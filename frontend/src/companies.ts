interface CompanyLabelFormat {
    company: string;
    rows: string[][];
}

export let labelFormats: Array<CompanyLabelFormat> = [
    {
        company: "Bell/ITT",
        rows: [["PN", "Rev"], ["DOM", "Job#"], ["P Date", "P Code"], ["Class"]],
    },
    {
        company: "Kidde",
        rows: [["PN", "Rev"], ["DOM", "Job#"]],
    },
    {
        company: "Ontic",
        rows: [["PN", "Rev"], ["Job#"]],
    },
    {
        company: "Biomet",
        rows: [["PN"], ["Rev"]],
    },
    {
        company: "Linvatec",
        rows: [["PN"], ["Rev"]],
    },
    {
        company: "Purolator",
        rows: [["PN"], ["Rev"]],
    },
    {
        company: "Curtiss Wright",
        rows: [["PN"]],
    },
    {
        company: "Lord",
        rows: [["PN"], ["Class"]],
    },
    {
        company: "Sierra Nevada",
        rows: [["PN", "Rev"], ["Job#"], ["Thread Insp."]],
    },
    {
        company: "Dover",
        rows: [["PN"], ["Rev"]],
    },
    {
        company: "Moog",
        rows: [["PN", "Rev"], ["Job#"], ["PO#"]],
    },
    {
        company: "SpaceX",
        rows: [["PN", "Rev"], ["Job#"], ["SN"], ["PO/Line"]],
    },
].sort((a, b) => a.company < b.company ? -1 : 1);
