const { RustProject } = require('@gplassard/projen-extensions');

const project = new RustProject({
    name: 'pretty-logs',
    cargo: {
        package: {
            authors: ["Gabriel Plassard <gabriel.plassard@gmail.com>"],
            version: '1.0.2',
            edition: "2021",
        },
        dependencies: {
            'atty': "0.2.14",
            'env_logger': "0.10.0",
            'log': "0.4.16",
            'structopt': "0.3.26",
        }
    }
});
project.addGitIgnore('.idea');
project.addGitIgnore('*.iml');
project.synth();
