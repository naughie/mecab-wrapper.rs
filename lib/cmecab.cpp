#include <mecab.h>

using namespace MeCab;

extern "C" const char *get_global_error() {
    return getLastError();
}

extern "C" void *new_model_argv(int argc, char **argv) {
    Model *model = createModel(argc, argv);
    void *void_model = (void *)model;
    return void_model;
}

extern "C" void *new_model_single(const char *arg) {
    Model *model = createModel(arg);
    void *void_model = (void *)model;
    return void_model;
}

extern "C" void delete_model(void *void_model) {
    Model *model = (Model *)void_model;
    delete model;
}

extern "C" void *dictionary_info(void *void_model) {
    Model *model = (Model *)void_model;
    const DictionaryInfo *info = model->dictionary_info();
    void *void_info = (void *)info;
    return void_info;
}

extern "C" const char *model_version() {
    return Model::version();
}

extern "C" int transition_cost(void *void_model, unsigned short rattr, unsigned short lattr) {
    Model *model = (Model *)void_model;
    return model->transition_cost(rattr, lattr);
}

extern "C" bool swap_model(void *void_model, void *void_new_model) {
    Model *model = (Model *)void_model;
    Model *new_model = (Model *)void_new_model;

    return model->swap(new_model);
}

extern "C" void *model_lookup(void *void_model, const char *begin, const char *end, void *void_lattice) {
    Model *model = (Model *)void_model;
    Lattice *lattice = (Lattice *)void_lattice;

    Node *node = model->lookup(begin, end, lattice);
    void *void_node = (void *)node;
    return void_node;
}

extern "C" void *new_tagger(void *void_model) {
    Model *model = (Model *)void_model;
    Tagger *tagger = model->createTagger();
    void *void_tagger = (void *)tagger;
    return void_tagger;
}

extern "C" void delete_tagger(void *void_tagger) {
    Tagger *tagger = (Tagger *)void_tagger;
    delete tagger;
}

extern "C" const char *tagger_what(void *void_tagger) {
    Tagger *tagger = (Tagger *)void_tagger;
    return tagger->what();
}

extern "C" const char *tagger_version() {
    return Tagger::version();
}

extern "C" void *new_lattice(void *void_model) {
    Model *model = (Model *)void_model;
    Lattice *lattice = model->createLattice();
    void *void_lattice = (void *)lattice;
    return void_lattice;
}

extern "C" void *new_lattice_standalone() {
    Lattice *lattice = Lattice::create();
    void *void_lattice = (void *)lattice;
    return void_lattice;
}

extern "C" void delete_lattice(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    delete lattice;
}

extern "C" const char *lattice_what(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->what();
}
extern "C" void set_lattice_what(void *void_lattice, const char *what) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_what(what);
}

extern "C" void set_sentence(void *void_lattice, const char *input, size_t len) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_sentence(input, len);
}

extern "C" bool parse(void *void_tagger, void *void_lattice) {
    Tagger *tagger = (Tagger *)void_tagger;
    Lattice *lattice = (Lattice *)void_lattice;

    return tagger->parse(lattice);
}

extern "C" const char *lattice_to_string(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;

    return lattice->toString();
}

extern "C" const char *lattice_to_string_alloc(void *void_lattice, char *buf, size_t size) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->toString(buf, size);
}

extern "C" const char *nbest_string(void *void_lattice, size_t n) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->enumNBestAsString(n);
}

extern "C" const char *nbest_string_alloc(void *void_lattice, size_t n, char *buf, size_t size) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->enumNBestAsString(n, buf, size);
}

extern "C" const char *node_string(void *void_lattice, const void *void_node) {
    Lattice *lattice = (Lattice *)void_lattice;
    const Node* node = (const Node*)void_node;
    return lattice->toString(node);
}

extern "C" const char *node_string_alloc(void *void_lattice, const void *void_node, char *buf, size_t size) {
    Lattice *lattice = (Lattice *)void_lattice;
    const Node* node = (const Node*)void_node;
    return lattice->toString(node, buf, size);
}

extern "C" void *bos_node(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;

    const Node* bos = lattice->bos_node();
    void *void_node = (void *)bos;
    return void_node;
}

extern "C" void *eos_node(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;

    const Node* eos = lattice->eos_node();
    void *void_node = (void *)eos;
    return void_node;
}

extern "C" int get_request_type(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->request_type();
}

extern "C" void set_request_type(void *void_lattice, int request_type) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_request_type(request_type);
}

extern "C" void add_request_type(void *void_lattice, int request_type) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->add_request_type(request_type);
}

extern "C" void remove_request_type(void *void_lattice, int request_type) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->remove_request_type(request_type);
}

extern "C" bool next_lattice(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->next();
}

extern "C" void clear_lattice(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->clear();
}

extern "C" bool is_available(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->is_available();
}

extern "C" const char *lattice_sentence(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->sentence();
}

extern "C" size_t lattice_sentence_size(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->size();
}

extern "C" double lattice_norm_factor(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->Z();
}

extern "C" void lattice_set_norm_factor(void *void_lattice, double norm) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_Z(norm);
}

extern "C" float lattice_theta(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->theta();
}

extern "C" void lattice_set_theta(void *void_lattice, float theta) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_theta(theta);
}

extern "C" bool lattice_has_constraint(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->has_constraint();
}

extern "C" int lattice_boundary_constraint(void *void_lattice, size_t pos) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->boundary_constraint(pos);
}

extern "C" void lattice_set_boundary_constraint(void *void_lattice, size_t pos, int boundary) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_boundary_constraint(pos, boundary);
}

extern "C" const char *lattice_feature_constraint(void *void_lattice, size_t pos) {
    Lattice *lattice = (Lattice *)void_lattice;
    return lattice->feature_constraint(pos);
}

extern "C" void lattice_set_feature_constraint(void *void_lattice, size_t begin_pos, size_t end_pos, const char *feature) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_feature_constraint(begin_pos, end_pos, feature);
}

extern "C" void lattice_set_result(void *void_lattice, const char *result) {
    Lattice *lattice = (Lattice *)void_lattice;
    lattice->set_result(result);
}

extern "C" void *new_node(void *void_lattice) {
    Lattice *lattice = (Lattice *)void_lattice;
    const Node* node = lattice->newNode();
    void *void_node = (void *)node;
    return void_node;
}

extern "C" void *next_node(void *void_node) {
    const Node* node = (const Node*)void_node;
    return node->next;
}
