interface CompanyLabelFormat {
    company: string;
    fields: string[];
    format: string;
}

export let labelFormats: Array<CompanyLabelFormat> = [
    {
        company: "Bell/ITT",
        fields: ["PN", "Rev", "DOM", "Job#", "P Date", "P Code", "Class"],
        format: "..,..,..,.",
    },
    {
        company: "Kidde",
        fields: ["PN", "Rev", "DOM", "Job#"],
        format: "..,..",
    },
    {
        company: "Ontic",
        fields: ["PN", "Rev", "Job#"],
        format: "..,.",
    },
    {
        company: "Biomet",
        fields: ["PN", "Rev"],
        format: ".,.",
    },
    {
        company: "Linvatec",
        fields: ["PN", "Rev"],
        format: ".,.",
    },
    {
        company: "Purolator",
        fields: ["PN", "Rev"],
        format: ".,.",
    },
    {
        company: "Curtiss Wright",
        fields: ["PN"],
        format: ".",
    },
    {
        company: "Lord",
        fields: ["PN", "Class"],
        format: ".,.",
    },
    {
        company: "Sierra Nevada",
        fields: ["PN", "Rev", "Job#", "Thread Insp."],
        format: "..,.,.",
    },
    {
        company: "Dover",
        fields: ["PN", "Rev"],
        format: ".,.",
    },
    {
        company: "Moog",
        fields: ["PN", "Rev", "Job#", "PO#"],
        format: "..,.,.",
    },
    {
        company: "SpaceX",
        fields: ["PN", "Rev", "Job#", "SN", "PO/Line"],
        format: "..,.,.,.",
    },
];