import go.Go;
import go.Log;
import go.net.Url;
import go.net.http.Dir;
import go.net.http.Httputil;
import go.github_com.gin_gonic.Gin;

import haxe.io.Bytes;
import haxe.io.BytesOutput;
import haxe.http.HttpStatus;

import EnvMacro.env;

function main() {
	final r = Gin._default();

	r.loadHTMLFiles('../../public/upload.html');

	r.GET("/", (c) -> {
		c.HTML(HttpStatus.OK, "upload.html", ["title" => "heart"]);
	});

	final backend_url = 'http://${env('BACKEND_IP', "0.0.0.0")}:${env('BACKEND_PORT', "2")}';
	final target = (Url.parse(backend_url))!;
	final reverse_proxy = Httputil.newSingleHostReverseProxy(target);
	r.POST("/upload", Gin.wrapH(reverse_proxy));
	
	final nginx_url = 'http://${env('NGINX_IP')}:${env('NGINX_PORT')}';
	r.GET("/data/*f", (c) -> {
		c.redirect(HttpStatus.Found, '${nginx_url}/data/${c.param("f")}');
	});
	
	final err = r.run('${env('FRONTEND_IP', '127.0.0.1')}:${env('FRONTEND_PORT', "3000")}');
	if (err != null) {
		Log.fatalf("failed to run server: %v", [err]);
	}
}
