PORTNAME=	tree-sitter-grammars
DISTVERSIONPREFIX=	v
DISTVERSION=	0.0.1-16
DISTVERSIONSUFFIX=	-g84fa125
CATEGORIES=	textproc
MASTER_SITES=	https://git.sr.ht/~ghishadow/tree-sitter-hare/archive/:hare
DISTFILES=	${HARE_HASH}${EXTRACT_SUFX}:hare

MAINTAINER=	youremail@example.com
COMMENT=	tree-sitter-grammars
WWW=		https://github.com/lapce/tree-sitter-grammars

# XXX to be created, should be easy
BUILD_DEPENDS=	tree-sitter:devel/tree-sitter-cli

USES=		cargo node:build

USE_GITHUB=	yes
GH_ACCOUNT=	lapce
# https://reviews.freebsd.org/D24231
# x11/ly/Makefile _git-submodules-to-gh-tuple
GH_TUPLE=	6cdh:tree-sitter-scheme:v0.4.0-21-gaf3af6c:ts1/grammars/tree-sitter-scheme \
		abreumatheus:tree-sitter-clojure:fdc969e:ts2/grammars/tree-sitter-clojure \
		alex-pinkus:tree-sitter-swift:0.3.6-10-g29541ac:ts3/grammars/tree-sitter-swift \
		elixir-lang:tree-sitter-elixir:v0.1.0-7-ga2861e8:ts4/grammars/tree-sitter-elixir \
		elm-tooling:tree-sitter-elm:v5.6.6:ts5/grammars/tree-sitter-elm \
		fwcd:tree-sitter-kotlin:0.2.11-123-g06a2f6e:ts6/grammars/tree-sitter-kotlin \
		ghishadow:tree-sitter-d:d129542:tsd/grammars/tree-sitter-d \
		Himujjal:tree-sitter-svelte:v0.11.0-1-g697bb51:ts7/grammars/tree-sitter-svelte \
		latex-lsp:tree-sitter-latex:v0.3.0-23-g2ae2021:ts8/grammars/tree-sitter-latex \
		Mathspy:tree-sitter-toml:v0.5.1-3-gae4cdb5:ts9/grammars/tree-sitter-toml \
		maxxnino:tree-sitter-zig:0d08703:ts10/grammars/tree-sitter-zig \
		MDeiml:tree-sitter-markdown:v0.1.6:ts11/grammars/tree-sitter-markdown \
		MichaHoffmann:tree-sitter-hcl:v1.1.0-2-gb553906:ts12/grammars/tree-sitter-hcl \
		oknozor:tree-sitter-sql:15dad0f:ts13/grammars/tree-sitter-sql \
		panekj:tree-sitter-dockerfile:25c71d6:ts14/grammars/tree-sitter-dockerfile \
		panekj:tree-sitter-nix:59fc471:ts15/grammars/tree-sitter-nix \
		panekj:tree-sitter-yaml:80c8d76:ts16/grammars/tree-sitter-yaml \
		r-lib:tree-sitter-r:remotes/origin/feature/help-assign-precedence-7-gc55f8b4:ts17/grammars/tree-sitter-r \
		RenjiSann:tree-sitter-xml:48a7c2b:ts18/grammars/tree-sitter-xml \
		syntacti:tree-sitter-css:v0.16.0-11-g397aa13:ts19/grammars/tree-sitter-css \
		syntacti:tree-sitter-lua:v0.0.10-10-ga29f646:ts20/grammars/tree-sitter-lua \
		szebniok:tree-sitter-wgsl:40259f3:ts21/grammars/tree-sitter-wgsl \
		theHamsta:tree-sitter-glsl:v0.1.4-20-g4780c2b:ts22/grammars/tree-sitter-glsl \
		tree-sitter:tree-sitter-bash:v0.20.0:ts23/grammars/tree-sitter-bash \
		tree-sitter:tree-sitter-c-sharp:v0.19.0-183-g1648e21:ts24/grammars/tree-sitter-c-sharp \
		tree-sitter:tree-sitter-c:v0.20.6:ts25/grammars/tree-sitter-c \
		tree-sitter:tree-sitter-cpp:v0.20.3-2-ga90f170:ts26/grammars/tree-sitter-cpp \
		tree-sitter:tree-sitter-go:v0.20.0:ts27/grammars/tree-sitter-go \
		tree-sitter:tree-sitter-haskell:v0.13.0-167-g9970682:ts28/grammars/tree-sitter-haskell \
		tree-sitter:tree-sitter-html:v0.19.0-20-ge5d7d7d:ts29/grammars/tree-sitter-html \
		tree-sitter:tree-sitter-java:v0.19.1-107-g0b3f9cf:ts30/grammars/tree-sitter-java \
		tree-sitter:tree-sitter-javascript:rust-0.20.0-25-gc69aaba:ts31/grammars/tree-sitter-javascript \
		tree-sitter:tree-sitter-json:v0.19.0-15-gca3f891:ts32/grammars/tree-sitter-json \
		tree-sitter:tree-sitter-ocaml:v0.20.4:ts33/grammars/tree-sitter-ocaml \
		tree-sitter:tree-sitter-php:v0.19.0-134-gce2c73a:ts34/grammars/tree-sitter-php \
		tree-sitter:tree-sitter-python:v0.20.4:ts35/grammars/tree-sitter-python \
		tree-sitter:tree-sitter-ql:v1.0.0-90-gbd08702:ts36/grammars/tree-sitter-ql \
		tree-sitter:tree-sitter-ruby:v0.19.0-117-gf257f3f:ts37/grammars/tree-sitter-ruby \
		tree-sitter:tree-sitter-rust:v0.19.0-68-g0a70e15:ts38/grammars/tree-sitter-rust \
		tree-sitter:tree-sitter-typescript:v0.20.2-26-gb1bf482:ts39/grammars/tree-sitter-typescript \
		uyha:tree-sitter-cmake:v0.4.1:ts40/grammars/tree-sitter-cmake \
		vantreeseba:tree-sitter-haxe:v0.3.4:ts41/grammars/tree-sitter-haxe \
		varlad:tree-sitter-julia:09aba7f:ts42/grammars/tree-sitter-julia \
		victorhqc:tree-sitter-prisma:v1.4.0:ts43/grammars/tree-sitter-prisma \
		VixieTSQ:tree-sitter-glimmer:v1.0.1-77-g7281cac:ts44/grammars/tree-sitter-glimmer \
		VixieTSQ:tree-sitter-scss:v1.0.0:ts45/grammars/tree-sitter-scss \
		WhatsApp:tree-sitter-erlang:0.1.0-1-g7aa24fe:ts46/grammars/tree-sitter-erlang \
		xiaoxin-sky:tree-sitter-vue:8bbcd4c:ts47/grammars/tree-sitter-vue \
		yusdacra:tree-sitter-protobuf:5aef38d:ts48/grammars/tree-sitter-protobuf

HARE_HASH=	7d9080c73eacb7b22900a71320058025615d57fa

CARGO_CARGOTOML=	${WRKSRC}/build-tree-sitter/Cargo.toml
CARGO_CARGOLOCK=	${WRKSRC}/build-tree-sitter/Cargo.lock
CARGO_INSTALL=		NO

# XXX is it needed?
USE_LDCONFIG=	YES

post-extract:
	${MV} ${WRKDIR}/tree-sitter-hare-${HARE_HASH}/* ${WRKSRC}/grammars/tree-sitter-hare

do-build:
	# we need to build build-tree-sitter first
	# see .github/workflows/release.yaml
	# this is ${CARGO_CARGO_RUN} minus "cd ${WRKSRC}"
	cd ${WRKSRC}/build-tree-sitter; \
		${SETENVI} ${WRK_ENV} ${MAKE_ENV} ${CARGO_ENV} CARGO_FREEBSD_PORTS_SKIP_GIT_UPDATE=1 ${CARGO} build --verbose --verbose ${CARGO_BUILD_ARGS}

	cd ${WRKSRC}; \
		${CARGO_TARGET_DIR}/${CARGO_BUILD_TARGET}/*/build-tree-sitter --output ./output

do-install:
	${INSTALL_LIB} ${WRKSRC}/output/*.so ${STAGEDIR}${PREFIX}/lib

.include <bsd.port.mk>
