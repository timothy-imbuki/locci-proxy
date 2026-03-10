// test-regex.js – Simulates locci-proxy regex routing
// Run with: node test-regex.js

const proxyRules = [
    { pattern: ".*\\.google\\.com", description: "Match any google subdomain" },
    { pattern: "^/api/v[0-9]/", description: "Match versioned API paths" }
];

const testCases = [
    { url: "https://mail.google.com/mail", host: "mail.google.com", path: "/mail" },
    { url: "https://example.com/api/v1/users", host: "example.com", path: "/api/v1/users" },
    { url: "https://bing.com", host: "bing.com", path: "/" }
];

function testProxyRegex(host, path) {
    console.log(`Testing Host: ${host}, Path: ${path}`);

    proxyRules.forEach(rule => {
        try {
            const fullUrl = `https://${host}${path}`;
            const regex = new RegExp(rule.pattern, 'i');
            const isMatch = regex.test(fullUrl);
            console.log(`  - Rule [${rule.pattern}]: ${isMatch ? "MATCHED ✅" : "NO MATCH ❌"}`);
        } catch (e) {
            console.error(`  - Invalid Regex Pattern: ${rule.pattern}`);
        }
    });
    console.log('----------------------------');
}

testCases.forEach(tc => testProxyRegex(tc.host, tc.path));
