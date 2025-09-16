from mitmproxy import http

def request(flow: http.HTTPFlow):
    print(">>>", flow.response.status_code, flow.response.reason)
    print(flow.request.headers)
    print(flow.request.get_text()[:50000], "\n")
