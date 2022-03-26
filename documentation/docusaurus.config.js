const path = require('path');

module.exports = {
    title: 'Stronghold',
    url: '/',
    baseUrl: '/',
    themes: ['@docusaurus/theme-classic'],
    themeConfig: {
        navbar: {
            items: [
                {
                    type: 'docsVersionDropdown',
                    docsPluginId: 'stronghold-rs',
                }
            ]
        }
    },
    plugins: [
        [
            '@docusaurus/plugin-content-docs',
            {
                id: 'stronghold-rs',
                path: path.resolve(__dirname, './docs'),
                routeBasePath: 'stronghold',
                sidebarPath: path.resolve(__dirname, './sidebars.js'),
                editUrl: 'https://github.com/iotaledger/stronghold/edit/dev/',
                remarkPlugins: [require('remark-code-import'), require('remark-remove-comments')],
                versions: {
                    current: {
                        label: 'Shimmer'
                    },
                    iota: {
                        label: 'IOTA'
                    }
                }
            }
        ],
    ],
    staticDirectories: [path.resolve(__dirname, './static')],
};