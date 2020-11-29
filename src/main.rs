use error_chain::error_chain;
use serde::Deserialize;
use structopt::StructOpt;

const API: &str = "https://api.covid19api.com/summary";

const COUNTRIES: [&str; 248] = [
	"barbados",
	"gibraltar",
	"lithuania",
	"malaysia",
	"nauru",
	"palestine",
	"qatar",
	"solomon-islands",
	"sri-lanka",
	"turks-and-caicos-islands",
	"vanuatu",
	"wallis-and-futuna-islands",
	"dominica",
	"gambia",
	"iran",
	"namibia",
	"tokelau",
	"guinea",
	"morocco",
	"tunisia",
	"azerbaijan",
	"honduras",
	"saint-martin-french-part",
	"chad",
	"christmas-island",
	"costa-rica",
	"bulgaria",
	"denmark",
	"malawi",
	"nepal",
	"switzerland",
	"canada",
	"china",
	"grenada",
	"andorra",
	"belgium",
	"burkina-faso",
	"cayman-islands",
	"estonia",
	"jersey",
	"montserrat",
	"oman",
	"cameroon",
	"luxembourg",
	"slovakia",
	"bosnia-and-herzegovina",
	"moldova",
	"brunei",
	"eritrea",
	"jordan",
	"liberia",
	"portugal",
	"ukraine",
	"indonesia",
	"kenya",
	"georgia",
	"iceland",
	"jamaica",
	"norfolk-island",
	"french-southern-territories",
	"guernsey",
	"paraguay",
	"tajikistan",
	"us-minor-outlying-islands",
	"uzbekistan",
	"american-samoa",
	"british-virgin-islands",
	"finland",
	"malta",
	"botswana",
	"israel",
	"saint-lucia",
	"spain",
	"argentina",
	"congo-kinshasa",
	"réunion",
	"saint-barthélemy",
	"zambia",
	"bahrain",
	"chile",
	"uruguay",
	"yemen",
	"austria",
	"bangladesh",
	"italy",
	"micronesia",
	"puerto-rico",
	"tuvalu",
	"ala-aland-islands",
	"afghanistan",
	"cambodia",
	"sao-tome-and-principe",
	"korea-north",
	"new-zealand",
	"turkey",
	"greenland",
	"hungary",
	"australia",
	"india",
	"san-marino",
	"united-kingdom",
	"comoros",
	"mauritania",
	"benin",
	"cote-divoire",
	"guadeloupe",
	"heard-and-mcdonald-islands",
	"kiribati",
	"mali",
	"northern-mariana-islands",
	"palau",
	"myanmar",
	"somalia",
	"trinidad-and-tobago",
	"turkmenistan",
	"french-guiana",
	"lao-pdr",
	"united-arab-emirates",
	"albania",
	"ireland",
	"burundi",
	"cape-verde",
	"greece",
	"guam",
	"guatemala",
	"korea-south",
	"niger",
	"panama",
	"saint-helena",
	"vietnam",
	"saint-kitts-and-nevis",
	"seychelles",
	"timor-leste",
	"bouvet-island",
	"el-salvador",
	"russia",
	"slovenia",
	"french-polynesia",
	"hong-kong-sar-china",
	"madagascar",
	"nigeria",
	"samoa",
	"british-indian-ocean-territory",
	"equatorial-guinea",
	"holy-see-vatican-city-state",
	"rwanda",
	"saint-vincent-and-the-grenadines",
	"virgin-islands",
	"congo-brazzaville",
	"guyana",
	"haiti",
	"marshall-islands",
	"singapore",
	"bhutan",
	"ghana",
	"mozambique",
	"antigua-and-barbuda",
	"cocos-keeling-islands",
	"cyprus",
	"latvia",
	"lebanon",
	"cook-islands",
	"isle-of-man",
	"libya",
	"netherlands",
	"new-caledonia",
	"niue",
	"thailand",
	"egypt",
	"faroe-islands",
	"south-georgia-and-the-south-sandwich-islands",
	"algeria",
	"brazil",
	"central-african-republic",
	"czech-republic",
	"ecuador",
	"gabon",
	"zimbabwe",
	"peru",
	"saint-pierre-and-miquelon",
	"kosovo",
	"tonga",
	"south-sudan",
	"colombia",
	"germany",
	"lesotho",
	"falkland-islands-malvinas",
	"saudi-arabia",
	"aruba",
	"swaziland",
	"united-states",
	"antarctica",
	"cuba",
	"monaco",
	"serbia",
	"anguilla",
	"maldives",
	"romania",
	"uganda",
	"japan",
	"belarus",
	"france",
	"western-sahara",
	"iraq",
	"norway",
	"philippines",
	"bahamas",
	"dominican-republic",
	"macao-sar-china",
	"bermuda",
	"pakistan",
	"pitcairn",
	"tanzania",
	"bolivia",
	"kazakhstan",
	"kyrgyzstan",
	"taiwan",
	"armenia",
	"svalbard-and-jan-mayen-islands",
	"sweden",
	"togo",
	"fiji",
	"martinique",
	"montenegro",
	"suriname",
	"venezuela",
	"angola",
	"macedonia",
	"mauritius",
	"mayotte",
	"senegal",
	"south-africa",
	"syria",
	"liechtenstein",
	"netherlands-antilles",
	"croatia",
	"djibouti",
	"mexico",
	"belize",
	"guinea-bissau",
	"mongolia",
	"poland",
	"ethiopia",
	"kuwait",
	"nicaragua",
	"papua-new-guinea",
	"sierra-leone",
	"sudan",
];

error_chain! {
	foreign_links {
		Io(std::io::Error);
		HttpRequest(reqwest::Error);
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct GlobalSummary {
	new_confirmed: u64,
	total_confirmed: u64,
	new_deaths: u64,
	total_deaths: u64,
	new_recovered: u64,
	total_recovered: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Country {
	country: String,
	country_code: String,
	slug: String,
	new_confirmed: u64,
	total_confirmed: u64,
	new_deaths: u64,
	total_deaths: u64,
	new_recovered: u64,
	total_recovered: u64,
	date: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Response {
	message: String,
	global: GlobalSummary,
	countries: Vec<Country>,
	date: String,
}

#[derive(StructOpt)]
struct Cli {
	location: String,
}

fn fetch_data() -> Result<Response> {
	let res = reqwest::blocking::get(API)?;
	let json_response = res.json::<Response>().unwrap();
	Ok(json_response)
}

fn print_global_summary(summary: GlobalSummary) {
	println!("========== GLOBAL COVID-19 CASES SUMMARY ==========");
	println!(
		"No. of new COVID-19 cases confirmed today: {}",
		summary.new_confirmed
	);
	println!(
		"Total no. of confirmed COVID-19 cases: {}",
		summary.total_confirmed
	);
	println!(
		"No. of new deaths caused by COVID-19 today: {}",
		summary.new_deaths
	);
	println!(
		"Total no. of deaths caused by COVID-19: {}",
		summary.total_deaths
	);
	println!(
		"No. of newly recovered COVID-19 patients today: {}",
		summary.new_recovered
	);
	println!(
		"Total no. of recovered COVID-19 patients: {}",
		summary.total_recovered
	);
}

fn print_country_summary(country: &Country) {
	println!(
		"========== {} COVID-19 CASES SUMMARY ==========",
		country.country.to_uppercase()
	);
	println!(
		"No. of new COVID-19 cases confirmed today: {}",
		country.new_confirmed
	);
	println!(
		"Total no. of confirmed COVID-19 cases: {}",
		country.total_confirmed
	);
	println!(
		"No. of new deaths caused by COVID-19 today: {}",
		country.new_deaths
	);
	println!(
		"Total no. of deaths caused by COVID-19: {}",
		country.total_deaths
	);
	println!(
		"No. of newly recovered COVID-19 patients today: {}",
		country.new_recovered
	);
	println!(
		"Total no. of recovered COVID-19 patients: {}",
		country.total_recovered
	);
}

fn main() {
	let args = Cli::from_args();
	let location_arg = &args.location[..];
	let summary_data = fetch_data().unwrap();
	if location_arg == "global" {
		let global_summary: GlobalSummary = summary_data.global;
		print_global_summary(global_summary);
	} else if COUNTRIES.iter().any(|&slug| slug == location_arg) {
		let country_summary: &Country = summary_data
			.countries
			.iter()
			.find(|&country| country.slug == location_arg)
			.unwrap();
		print_country_summary(country_summary);
	} else {
		println!("Please provide either the global command or a valid country slug/code from https://api.covid19api.com/countries");
	}
}
