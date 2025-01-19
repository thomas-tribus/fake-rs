use crate::{faker::impls::address::CityNameGenFn, locales::Data};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct PT_PT;

impl Data for PT_PT {
    const NAME_FIRST_NAME: &'static [&'static str] = &[
        "Abel",
        "Adriana",
        "Adriano",
        "Alberto",
        "Alexandra",
        "Alexandre",
        "Ana",
        "André",
        "António",
        "Artur",
        "Beatriz",
        "Bruno",
        "Carla",
        "Carlos",
        "Carolina",
        "Catarina",
        "Cristina",
        "Daniel",
        "David",
        "Diana",
        "Diogo",
        "Eduardo",
        "Fernando",
        "Filipa",
        "Francisco",
        "Gonçalo",
        "Guilherme",
        "Helena",
        "Hugo",
        "Inês",
        "Isabel",
        "João",
        "Jorge",
        "José",
        "Leonor",
        "Lia",
        "Liliana",
        "Luís",
        "Manuel",
        "Margarida",
        "Maria",
        "Mário",
        "Miguel",
        "Nuno",
        "Paulo",
        "Pedro",
        "Rafael",
        "Ricardo",
        "Rita",
        "Rosa",
        "Rui",
        "Sara",
        "Sofia",
        "Teresa",
        "Tiago",
        "Tomás",
        "Vasco",
        "Afonso",
        "Alice",
        "Amélia",
        "Bernardo",
        "Clara",
        "Dinis",
        "Duarte",
        "Eva",
        "Gabriel",
        "Gustavo",
        "Íris",
        "Joana",
        "Joaquim",
        "Laura",
        "Lucas",
        "Luísa",
        "Madalena",
        "Mafalda",
        "Martim",
        "Matilde",
        "Melissa",
        "Nicole",
        "Noa",
        "Rafaela",
        "Raquel",
        "Rodrigo",
        "Salvador",
        "Santiago",
        "Simão",
        "Vicente",
    ];

    const NAME_LAST_NAME: &'static [&'static str] = &[
        "Abreu",
        "Almeida",
        "Alves",
        "Araújo",
        "Azevedo",
        "Barbosa",
        "Barros",
        "Batista",
        "Borges",
        "Branco",
        "Caldas",
        "Campos",
        "Cardoso",
        "Carvalho",
        "Castro",
        "Coelho",
        "Correia",
        "Costa",
        "Cruz",
        "Cunha",
        "Dias",
        "Domingues",
        "Esteves",
        "Faria",
        "Fernandes",
        "Ferreira",
        "Figueiredo",
        "Freitas",
        "Garcia",
        "Gaspar",
        "Gomes",
        "Gonçalves",
        "Guerra",
        "Henriques",
        "Jesus",
        "Leal",
        "Lima",
        "Lopes",
        "Lourenço",
        "Machado",
        "Magalhães",
        "Marques",
        "Martins",
        "Matos",
        "Melo",
        "Mendes",
        "Meireles",
        "Miranda",
        "Monteiro",
        "Morais",
        "Moreira",
        "Mota",
        "Nunes",
        "Oliveira",
        "Pacheco",
        "Paiva",
        "Pascoal",
        "Paixão",
        "Pereira",
        "Pinheiro",
        "Pinto",
        "Pires",
        "Ramos",
        "Reis",
        "Ribeiro",
        "Rocha",
        "Rodrigues",
        "Rosa",
        "Santos",
        "Santana",
        "Saraiva",
        "Silva",
        "Simões",
        "Soares",
        "Sousa",
        "Teixeira",
        "Vaz",
        "Vicente",
        "Abrantes",
        "Aguiar",
        "Amaral",
        "Antunes",
        "Baptista",
        "Barreto",
        "Bessa",
        "Carneiro",
        "Carreira",
        "Coutinho",
        "Duarte",
        "Fonseca",
        "Godinho",
        "Guerreiro",
        "Leite",
        "Macedo",
        "Nascimento",
        "Neto",
        "Nogueira",
        "Pais",
        "Queirós",
        "Ramos",
        "Raposo",
        "Salgueiro",
        "Sampaio",
        "Sequeira",
        "Tavares",
        "Vale",
        "Valente",
        "Vieira",
    ];

    const INTERNET_FREE_EMAIL_PROVIDER: &'static [&'static str] = &[
        "gmail.com",
        "hotmail.com",
        "yahoo.com",
        "sapo.pt",
        "netcabo.pt",
        "clix.pt",
        "iol.pt",
        "live.com",
        "meo.pt",
        "outlook.com",
        "portugalmail.pt",
        "icloud.com",
    ];
    const INTERNET_DOMAIN_SUFFIX: &'static [&'static str] = &["pt", "com", "net", "org"];

    const PHONE_NUMBER_FORMATS: &'static [&'static str] = &[
        "+351 2## ### ####",
        "+351 2## #######",
        "+351 9## ### ####",
        "+351 9## #######",
    ];
    const PHONE_CELL_NUMBER_FORMATS: &'static [&'static str] = &[
        "+351 91# ### ####",
        "+351 92# ### ####",
        "+351 93# ### ####",
        "+351 96# ### ####",
    ];

    const CHRONO_DEFAULT_TIME_FORMAT: &'static str = "%H:%M:%S";
    const CHRONO_DEFAULT_DATE_FORMAT: &'static str = "%d-%m-%Y";
    const CHRONO_DEFAULT_DATETIME_FORMAT: &'static str = "%d-%m-%Y %H:%M:%S";

    const TIME_DEFAULT_DATE_FORMAT: &'static str = "[day]-[month]-[year]";
    const TIME_DEFAULT_DATETIME_FORMAT: &'static str =
        "[day]-[month]-[year] [hour]:[minute]:[second]";

    const ADDRESS_BUILDING_NUMBER_FORMATS: &'static [&'static str] = &["nº ###", "nº ##", "nº #"];
    const ADDRESS_CITY_PREFIX: &'static [&'static str] =
        &["Vila", "São", "Santa", "Santo", "Póvoa", "Alto"];
    const ADDRESS_CITY_SUFFIX: &'static [&'static str] = &[
        "do Castelo",
        "da Feira",
        "de Cima",
        "de Baixo",
        "de Riba",
        "de Sousa",
    ];
    const ADDRESS_COUNTRY: &'static [&'static str] = &[
        "Afeganistão",
        "Albânia",
        "Argélia",
        "Samoa Americana",
        "Andorra",
        "Angola",
        "Anguila",
        "Antártida (o território a Sul de 60 graus)",
        "Antígua e Barbuda",
        "Argentina",
        "Arménia",
        "Aruba",
        "Austrália",
        "Áustria",
        "Azerbaijão",
        "Bahamas",
        "Bahrein",
        "Bangladesh",
        "Barbados",
        "Bielorrússia",
        "Bélgica",
        "Belize",
        "Benim",
        "Bermudas",
        "Butão",
        "Bolívia",
        "Bósnia e Herzegovina",
        "Botswana",
        "Ilha Bouvet",
        "Brasil",
        "Território Britânico do Oceano Índico",
        "Brunei",
        "Bulgária",
        "Burkina Faso",
        "Burundi",
        "Camboja",
        "Camarões",
        "Canadá",
        "Cabo Verde",
        "Ilhas Cayman",
        "República Centro-Africana",
        "Chade",
        "Chile",
        "China",
        "Ilha Christmas",
        "Ilhas Cocos (Keeling)",
        "Colômbia",
        "Comores",
        "Congo",
        "Ilhas Cook",
        "Costa Rica",
        "Costa do Marfim",
        "Croácia",
        "Cuba",
        "Chipre",
        "República Checa",
        "Dinamarca",
        "Djibuti",
        "Dominica",
        "República Dominicana",
        "Equador",
        "Egito",
        "El Salvador",
        "Guiné Equatorial",
        "Eritreia",
        "Estónia",
        "Etiópia",
        "Ilhas Faroé",
        "Ilhas Malvinas",
        "Fiji",
        "Finlândia",
        "França",
        "Guiana Francesa",
        "Polinésia Francesa",
        "Terras Austrais e Antárticas Francesas",
        "Gabão",
        "Gâmbia",
        "Geórgia",
        "Alemanha",
        "Gana",
        "Gibraltar",
        "Grécia",
        "Gronelândia",
        "Granada",
        "Guadalupe",
        "Guam",
        "Guatemala",
        "Guernsey",
        "Guiné",
        "Guiné-Bissau",
        "Guiana",
        "Haiti",
        "Ilha Heard e Ilhas McDonald",
        "Cidade do Vaticano",
        "Honduras",
        "Hong Kong",
        "Hungria",
        "Islândia",
        "Índia",
        "Indonésia",
        "Irão",
        "Iraque",
        "Irlanda",
        "Ilha de Man",
        "Israel",
        "Itália",
        "Jamaica",
        "Japão",
        "Jersey",
        "Jordânia",
        "Cazaquistão",
        "Quénia",
        "Kiribati",
        "Coreia do Norte",
        "Coreia do Sul",
        "Kuwait",
        "Quirguistão",
        "Laos",
        "Letónia",
        "Líbano",
        "Lesoto",
        "Libéria",
        "Líbia",
        "Liechtenstein",
        "Lituânia",
        "Luxemburgo",
        "Macau",
        "Macedónia",
        "Madagáscar",
        "Malawi",
        "Malásia",
        "Maldivas",
        "Mali",
        "Malta",
        "Ilhas Marshall",
        "Martinica",
        "Mauritânia",
        "Maurícia",
        "Mayotte",
        "México",
        "Micronésia",
        "Moldávia",
        "Mónaco",
        "Mongólia",
        "Montenegro",
        "Montserrat",
        "Marrocos",
        "Moçambique",
        "Myanmar",
        "Namíbia",
        "Nauru",
        "Nepal",
        "Antilhas Holandesas",
        "Países Baixos",
        "Nova Caledónia",
        "Nova Zelândia",
        "Nicarágua",
        "Níger",
        "Nigéria",
        "Niue",
        "Ilha Norfolk",
        "Ilhas Marianas do Norte",
        "Noruega",
        "Omã",
        "Paquistão",
        "Palau",
        "Palestina",
        "Panamá",
        "Papua Nova Guiné",
        "Paraguai",
        "Peru",
        "Filipinas",
        "Ilhas Pitcairn",
        "Polónia",
        "Portugal",
        "Porto Rico",
        "Qatar",
        "Reunião",
        "Roménia",
        "Rússia",
        "Ruanda",
        "São Bartolomeu",
        "Santa Helena",
        "São Cristóvão e Neves",
        "Santa Lúcia",
        "São Martinho",
        "São Pedro e Miquelão",
        "São Vicente e Granadinas",
        "Samoa",
        "São Marino",
        "São Tomé e Príncipe",
        "Arábia Saudita",
        "Senegal",
        "Sérvia",
        "Seychelles",
        "Serra Leoa",
        "Singapura",
        "Eslováquia",
        "Eslovénia",
        "Ilhas Salomão",
        "Somália",
        "África do Sul",
        "Geórgia do Sul e Sandwich do Sul",
        "Espanha",
        "Sri Lanka",
        "Sudão",
        "Suriname",
        "Svalbard e Jan Mayen",
        "Suazilândia",
        "Suécia",
        "Suíça",
        "Síria",
        "Taiwan",
        "Tajiquistão",
        "Tanzânia",
        "Tailândia",
        "Timor-Leste",
        "Togo",
        "Tokelau",
        "Tonga",
        "Trinidad e Tobago",
        "Tunísia",
        "Turquia",
        "Turquemenistão",
        "Ilhas Turcas e Caicos",
        "Tuvalu",
        "Uganda",
        "Ucrânia",
        "Emirados Árabes Unidos",
        "Reino Unido",
        "Estados Unidos",
        "Ilhas Menores Distantes dos Estados Unidos",
        "Uruguai",
        "Uzbequistão",
        "Vanuatu",
        "Venezuela",
        "Vietname",
        "Ilhas Virgens Britânicas",
        "Ilhas Virgens Americanas",
        "Wallis e Futuna",
        "Saara Ocidental",
        "Iémen",
        "Zâmbia",
        "Zimbabué",
    ];
    const ADDRESS_SECONDARY_ADDR_TYPE: &'static [&'static str] = &["Apt.", "Suite", "Piso"];
    const ADDRESS_STATE: &'static [&'static str] = &[
        "Aveiro",
        "Beja",
        "Braga",
        "Bragança",
        "Castelo Branco",
        "Coimbra",
        "Évora",
        "Faro",
        "Guarda",
        "Leiria",
        "Lisboa",
        "Portalegre",
        "Porto",
        "Santarém",
        "Setúbal",
        "Viana do Castelo",
        "Vila Real",
        "Viseu",
        "Madeira",
        "Açores",
    ];
    const ADDRESS_STATE_ABBR: &'static [&'static str] = &[
        "AV", "BJ", "BR", "BG", "CB", "CM", "ÉV", "FR", "GD", "LR", "LS", "PA", "PO", "ST", "SV",
        "VC", "VR", "VS", "MD", "AC",
    ];
    const ADDRESS_STREET_SUFFIX: &'static [&'static str] = &[
        "Alameda", "Avenida", "Calçada", "Estrada", "Largo", "Praça", "Rua", "Travessa",
    ];
    const ADDRESS_STREET_TPL: &'static str = "{StreetSuffix} {FirstName} {LastName}";
    const ADDRESS_TIME_ZONE: &'static [&'static str] =
        &["Atlantic/Azores", "Atlantic/Madeira", "Europe/Lisbon"];
    const ADDRESS_ZIP_FORMATS: &'static [&'static str] = &["####-###"];
    const ADDRESS_POSTCODE_FORMATS: &'static [&'static str] = &["####-###"];

    // Add currency related constants
    const CURRENCY_NAME: &'static [&'static str] = &[
        "Euro",
        "Dólar Americano",
        "Libra Esterlina",
        "Iene",
        "Franco Suíço",
        "Dólar Australiano",
        "Dólar Canadiano",
        "Yuan Chinês",
        "Real Brasileiro",
        "Peso Mexicano",
    ];

    const CURRENCY_CODE: &'static [&'static str] = &[
        "EUR", "USD", "GBP", "JPY", "CHF", "AUD", "CAD", "CNY", "BRL", "MXN",
    ];

    const CURRENCY_SYMBOL: &'static [&'static str] =
        &["€", "$", "£", "¥", "Fr.", "A$", "C$", "¥", "R$", "$"];

    // Add company related constants
    const COMPANY_SUFFIX: &'static [&'static str] = &[
        "S.A.",
        "Lda.",
        "SGPS",
        "e Filhos",
        "Unipessoal",
        "Sociedade Unipessoal",
        "Group",
    ];

    const NAME_TITLE: &'static [&'static str] = &[
        "Sr.", "Sra.", "Dr.", "Dra.", "Eng.", "Engª.", "Prof.", "Profª.",
    ];
    const NAME_SUFFIX: &'static [&'static str] = &["Jr.", "Filho", "Neto"];
    const NAME_TPL: &'static str = "{FirstName} {LastName}";
    const NAME_WITH_TITLE_TPL: &'static str = "{Title} {FirstName} {LastName}";

    const JOB_SENIORITY: &'static [&'static str] = &[
        "Principal",
        "Sénior",
        "Júnior",
        "Corporativo",
        "Nacional",
        "Regional",
        "Distrital",
        "Central",
        "Global",
        "Internacional",
        "Chefe",
    ];

    const JOB_FIELD: &'static [&'static str] = &[
        "Marketing",
        "Informática",
        "Contabilidade",
        "Administração",
        "Publicidade",
        "Banca",
        "Serviços Comunitários",
        "Construção",
        "Consultoria",
        "Design",
        "Educação",
        "Agricultura",
        "Governo",
        "Saúde",
        "Hotelaria",
        "Jurídico",
        "Indústria",
        "Mineração",
        "Imobiliário",
        "Retalho",
        "Vendas",
        "Tecnologia",
    ];

    const JOB_POSITION: &'static [&'static str] = &[
        "Supervisor",
        "Associado",
        "Executivo",
        "Coordenador",
        "Oficial",
        "Gerente",
        "Engenheiro",
        "Especialista",
        "Diretor",
        "Administrador",
        "Arquiteto",
        "Analista",
        "Designer",
        "Planeador",
        "Técnico",
        "Desenvolvedor",
        "Produtor",
        "Consultor",
        "Assistente",
        "Facilitador",
        "Agente",
        "Representante",
        "Estrategista",
    ];

    const JOB_TITLE_TPL: &'static str = "{Seniority} {Field} {Position}";

    const ADDRESS_CITY_TPL: &'static str = "{CityName} {CitySuffix}";
    const ADDRESS_CITY_WITH_PREFIX_TPL: &'static str = "{CityPrefix} {CityName} {CitySuffix}";
    const ADDRESS_SECONDARY_ADDR_TPL: &'static str = "{SecondaryAddrType} {Number}";

    const COMPANY_NAME_TPLS: &'static [&'static str] =
        &["{Name_1} {Suffix}", "{Name_1} e {Name_2} {Suffix}"];
    const COMPANY_BUZZWORD_HEAD: &'static [&'static str] = &[
        "Adaptável",
        "Avançado",
        "Automatizado",
        "Centralizado",
        "Compatível",
        "Configurável",
        "Digital",
        "Distribuído",
        "Diversificado",
        "Exclusivo",
        "Expandido",
        "Focado",
        "Fundamental",
        "Inovador",
        "Integrado",
        "Intuitivo",
        "Otimizado",
        "Organizado",
        "Progressivo",
        "Robusto",
        "Versátil",
        "Virtual",
    ];

    const COMPANY_BUZZWORD_MIDDLE: &'static [&'static str] = &[
        "24 horas",
        "global",
        "digital",
        "dinâmico",
        "empresarial",
        "eficiente",
        "flexível",
        "integrado",
        "inovador",
        "inteligente",
        "móvel",
        "proativo",
        "profissional",
        "regional",
        "responsivo",
        "sustentável",
        "tecnológico",
    ];

    const COMPANY_BUZZWORD_TAIL: &'static [&'static str] = &[
        "solução",
        "serviço",
        "sistema",
        "estratégia",
        "tecnologia",
        "metodologia",
        "arquitetura",
        "infraestrutura",
        "aplicação",
        "interface",
        "iniciativa",
        "plataforma",
        "abordagem",
        "inovação",
        "paradigma",
        "visão",
    ];

    const COMPANY_CATCH_PHASE_TPL: &'static str = "{Head} {Middle} {Tail}";

    const COMPANY_BS_VERBS: &'static [&'static str] = &[
        "implementar",
        "utilizar",
        "integrar",
        "otimizar",
        "evoluir",
        "transformar",
        "desenvolver",
        "inovar",
        "liderar",
        "maximizar",
        "potenciar",
        "expandir",
    ];

    const COMPANY_BS_ADJ: &'static [&'static str] = &[
        "eficiente",
        "proativo",
        "robusto",
        "revolucionário",
        "escalável",
        "inovador",
        "intuitivo",
        "estratégico",
        "integrado",
        "digital",
        "dinâmico",
        "global",
    ];

    const COMPANY_BS_NOUNS: &'static [&'static str] = &[
        "sinergias",
        "soluções",
        "paradigmas",
        "mercados",
        "parcerias",
        "infraestruturas",
        "plataformas",
        "iniciativas",
        "canais",
        "comunidades",
        "tecnologias",
        "metodologias",
    ];

    const COMPANY_BS_TPL: &'static str = "{Verb} {Adj} {Noun}";
}

impl CityNameGenFn for PT_PT {}